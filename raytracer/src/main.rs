use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;
use std::{fs::File, process::exit};

mod aabb;
mod aarect;
mod boxes;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rt_weekend;
mod sphere;
mod texture;
mod vec3;

use aarect::{XyRect, XzRect, YzRect};
use boxes::Box;
use bvh::BVHNode;
use camera::Camera;
use pdf::{CosinePdf, HittablePdf, Pdf};
// use constant_medium::ConstantMedium;
use hittable::{FlipFace, HitRecord, Hittable, RotateY, Translate};
use hittable_list::HittableList;
use image::GenericImageView;
//use material::DiffuseLight;
// use material::{Dielectric, Lambertian, Material, Metal};
use material::{DiffuseLight, Lambertian, Material};
use moving_sphere::MovingSphere;
use ray::Ray;
use rt_weekend::{random_double, random_double_range};
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use vec3::{dot, Color3, Point3, Vec3};

use std::sync::mpsc;
use std::thread;

pub fn hit_sphere(center: &Point3, radius: &f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - *center;
    let a: f64 = r.direction().length_squared();
    let b: f64 = 2.0 * vec3::dot(&oc, &r.direction());
    let c: f64 = oc.length_squared() - radius * radius;
    let det: f64 = b * b - 4.0 * a * c;
    if det < 0.0 {
        -1.0
    } else {
        (-b - det.sqrt()) / (2.0 * a)
    }
}

pub fn ray_color(
    r: &Ray,
    background: &Color3,
    world: &dyn Hittable,
    lights: Arc<dyn Hittable>,
    depth: i32,
) -> Color3 {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Color3::new();
    }
    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered: Ray = Ray::new();
    // let attenuation: Color3 = Color3::new();
    let emitted = rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .emitted(&r, &rec, rec.u, rec.v, &rec.p);

    let mut pdf = 0.0;
    let mut albedo: Color3 = Color3::new();
    if !rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .scatter(r, &rec, &mut albedo, &mut scattered, &mut pdf)
    {
        return emitted;
    }

    // let on_light = Point3::construct(&[
    //     random_double_range(213.0, 243.0),
    //     554.0,
    //     random_double_range(227.0, 332.0),
    // ]);
    // let mut to_light = on_light - rec.p;
    // let distance_squared = to_light.length_squared();
    // to_light = to_light.unit();
    // if dot(&to_light, &rec.normal) < 0.0 {
    //     return emitted;
    // }

    // let light_area = (343.0 - 213.0) * (332.0 - 227.0);
    // let light_cosine = to_light.y().abs();
    // if (light_cosine < 0.0000001) {
    //     return emitted;
    // }
    // pdf = distance_squared / (light_cosine * light_area);
    // scattered = Ray::construct(&rec.p, &to_light, r.time());

    // let p = CosinePdf::construct(&rec.normal);
    // scattered = Ray::construct(&rec.p, &p.generate(), r.time());
    // pdf = p.value(&scattered.direction());

    let light_pdf = HittablePdf::construct(lights.clone(), &rec.p);
    scattered = Ray::construct(&rec.p, &light_pdf.generate(), r.time());
    pdf = light_pdf.value(&scattered.direction());

    emitted
        + albedo
            * rec
                .mat_ptr
                .as_ref()
                .unwrap()
                .scattering_pdf(r, &rec, &scattered)
            * ray_color(&scattered, background, world, lights, depth - 1)
            / pdf
}

pub fn write_color(pixel_color: &Color3, samples_per_pixel: u32) -> [u8; 3] {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    // Divide the color by the number of samples.
    let scale: f64 = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    [
        (256.0 * rt_weekend::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * rt_weekend::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * rt_weekend::clamp(b, 0.0, 0.999)) as u8,
    ]
}

// fn random_scene() -> HittableList {
//     let mut world = HittableList::new();

//     // let ground_material = Arc::new(Lambertian::construct(&Color3::construct(&[0.5, 0.5, 0.5])));
//     // world.add(Arc::new(Sphere::construct(
//     //     &Point3::construct(&[0.0, -1000.0, 0.0]),
//     //     1000.0,
//     //     ground_material,
//     // )));
//     let checker = Arc::new(CheckerTexture::construct_color(
//         &Color3::construct(&[0.2, 0.3, 0.1]),
//         &Color3::construct(&[0.9, 0.9, 0.9]),
//     ));
//     world.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, -1000.0, 0.0]),
//         1000.0,
//         Arc::new(Lambertian::construct_texture(checker)),
//     )));

//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_double();
//             let center = Point3::construct(&[
//                 a as f64 + 0.9 * random_double(),
//                 0.2,
//                 b as f64 + 0.9 * random_double(),
//             ]);

//             if (center - Point3::construct(&[4.0, 0.2, 0.0])).length() > 0.9 {
//                 let sphere_material: Arc<dyn Material>;
//                 if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = Color3::random() * Color3::random();
//                     sphere_material = Arc::new(Lambertian::construct(&albedo));
//                     let center2 =
//                         center + Vec3::construct(&[0.0, random_double_range(0.0, 0.5), 0.0]);
//                     world.add(Arc::new(MovingSphere::construct(
//                         &center,
//                         &center2,
//                         0.0,
//                         1.0,
//                         0.2,
//                         sphere_material,
//                     )));
//                 }
//             }
//         }
//     }

//     let material1 = Arc::new(Dielectric::construct(1.5));
//     world.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 1.0, 0.0]),
//         1.0,
//         material1,
//     )));

//     let material2 = Arc::new(Lambertian::construct(&Color3::construct(&[0.4, 0.2, 0.1])));
//     world.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[-4.0, 1.0, 0.0]),
//         1.0,
//         material2,
//     )));

//     let material3 = Arc::new(Metal::construct(&Color3::construct(&[0.7, 0.6, 0.5]), 0.0));
//     world.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[4.0, 1.0, 0.0]),
//         1.0,
//         material3,
//     )));

//     world
// }

// pub fn two_spheres() -> HittableList {
//     let mut objects = HittableList::new();

//     let checker = Arc::new(CheckerTexture::construct_color(
//         &Color3::construct(&[0.2, 0.3, 0.1]),
//         &Color3::construct(&[0.9, 0.9, 0.9]),
//     ));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, -10.0, 0.0]),
//         10.0,
//         Arc::new(Lambertian::construct_texture(checker.clone())),
//     )));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 10.0, 0.0]),
//         10.0,
//         Arc::new(Lambertian::construct_texture(checker)),
//     )));

//     objects
// }

// pub fn two_perlin_spheres() -> HittableList {
//     let mut objects = HittableList::new();
//     let pertext = Arc::new(NoiseTexture::construct(4.0));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, -1000.0, 0.0]),
//         1000.0,
//         Arc::new(Lambertian::construct_texture(pertext.clone())),
//     )));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 2.0, 0.0]),
//         2.0,
//         Arc::new(Lambertian::construct_texture(pertext)),
//     )));
//     objects
// }
// pub fn earth() -> HittableList {
//     let img = image::open("earthmap.jpg").expect("Failed to open image");
//     let width: u32 = img.width();
//     let height: u32 = img.height();
//     let mut data: Vec<u8> = Vec::new();
//     for (_x, _y, pixel) in img.pixels() {
//         let rgba = pixel.0;
//         let (r, g, b) = (rgba[0], rgba[1], rgba[2]);
//         data.push(r);
//         data.push(g);
//         data.push(b);
//     }
//     let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::construct(&data, width, height));
//     let earth_surface = Arc::new(Lambertian::construct_texture(earth_texture));
//     let globe = Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 0.0, 0.0]),
//         2.0,
//         earth_surface,
//     ));
//     HittableList::construct(globe)
// }
// pub fn simple_light() -> HittableList {
//     let mut objects = HittableList::new();

//     let pertext = Arc::new(NoiseTexture::construct(4.0));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, -1000.0, 0.0]),
//         1000.0,
//         Arc::new(Lambertian::construct_texture(pertext.clone())),
//     )));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 2.0, 0.0]),
//         2.0,
//         Arc::new(Lambertian::construct_texture(pertext)),
//     )));

//     let difflight = Arc::new(DiffuseLight::construct_color(&Color3::construct(&[
//         4.0, 4.0, 4.0,
//     ])));
//     objects.add(Arc::new(XyRect::construct(
//         3.0,
//         5.0,
//         1.0,
//         3.0,
//         -2.0,
//         difflight.clone(),
//     )));

//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 7.0, 0.0]),
//         2.0,
//         difflight,
//     )));

//     objects
// }

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::construct(&Color3::construct(&[
        0.65, 0.05, 0.05,
    ])));
    let white = Arc::new(Lambertian::construct(&Color3::construct(&[
        0.73, 0.73, 0.73,
    ])));
    let green = Arc::new(Lambertian::construct(&Color3::construct(&[
        0.12, 0.45, 0.15,
    ])));
    let light = Arc::new(DiffuseLight::construct_color(&Color3::construct(&[
        15.0, 15.0, 15.0,
    ])));

    objects.add(Arc::new(YzRect::construct(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(Arc::new(YzRect::construct(
        0.0, 555.0, 0.0, 555.0, 0.0, red,
    )));
    objects.add(Arc::new(FlipFace::construct(Arc::new(XzRect::construct(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.add(Arc::new(XzRect::construct(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::construct(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::construct(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    // objects.add(Arc::new(Box::construct(
    //     &Point3::construct(&[130.0, 0.0, 65.0]),
    //     &Point3::construct(&[295.0, 165.0, 230.0]),
    //     white.clone(),
    // )));
    // objects.add(Arc::new(Box::construct(
    //     &Point3::construct(&[265.0, 0.0, 295.0]),
    //     &Point3::construct(&[430.0, 330.0, 460.0]),
    //     white.clone(),
    // )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Box::construct(
        &Point3::construct(&[0.0, 0.0, 0.0]),
        &Point3::construct(&[165.0, 330.0, 165.0]),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::construct(box1, 15.0));
    box1 = Arc::new(Translate::construct(
        box1,
        &Vec3::construct(&[265.0, 0.0, 295.0]),
    ));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Box::construct(
        &Point3::construct(&[0.0, 0.0, 0.0]),
        &Point3::construct(&[165.0, 165.0, 165.0]),
        white,
    ));
    box2 = Arc::new(RotateY::construct(box2, -18.0));
    box2 = Arc::new(Translate::construct(
        box2,
        &Vec3::construct(&[130.0, 0.0, 65.0]),
    ));
    objects.add(box2);

    objects
}

// pub fn cornell_smoke() -> HittableList {
//     let mut objects = HittableList::new();

//     let red = Arc::new(Lambertian::construct(&Color3::construct(&[
//         0.65, 0.05, 0.05,
//     ])));
//     let white = Arc::new(Lambertian::construct(&Color3::construct(&[
//         0.73, 0.73, 0.73,
//     ])));
//     let green = Arc::new(Lambertian::construct(&Color3::construct(&[
//         0.12, 0.45, 0.15,
//     ])));
//     let light = Arc::new(DiffuseLight::construct_color(&Color3::construct(&[
//         7.0, 7.0, 7.0,
//     ])));

//     objects.add(Arc::new(YzRect::construct(
//         0.0, 555.0, 0.0, 555.0, 555.0, green,
//     )));
//     objects.add(Arc::new(YzRect::construct(
//         0.0, 555.0, 0.0, 555.0, 0.0, red,
//     )));
//     objects.add(Arc::new(XzRect::construct(
//         113.0, 443.0, 127.0, 432.0, 554.0, light,
//     )));
//     objects.add(Arc::new(XzRect::construct(
//         0.0,
//         555.0,
//         0.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )));
//     objects.add(Arc::new(XzRect::construct(
//         0.0,
//         555.0,
//         0.0,
//         555.0,
//         0.0,
//         white.clone(),
//     )));
//     objects.add(Arc::new(XyRect::construct(
//         0.0,
//         555.0,
//         0.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )));

//     let mut box1: Arc<dyn Hittable> = Arc::new(Box::construct(
//         &Point3::construct(&[0.0, 0.0, 0.0]),
//         &Point3::construct(&[165.0, 330.0, 165.0]),
//         white.clone(),
//     ));
//     box1 = Arc::new(RotateY::construct(box1, 15.0));
//     box1 = Arc::new(Translate::construct(
//         box1,
//         &Vec3::construct(&[265.0, 0.0, 295.0]),
//     ));

//     let mut box2: Arc<dyn Hittable> = Arc::new(Box::construct(
//         &Point3::construct(&[0.0, 0.0, 0.0]),
//         &Point3::construct(&[165.0, 165.0, 165.0]),
//         white,
//     ));
//     box2 = Arc::new(RotateY::construct(box2, -18.0));
//     box2 = Arc::new(Translate::construct(
//         box2,
//         &Vec3::construct(&[130.0, 0.0, 65.0]),
//     ));

//     objects.add(Arc::new(ConstantMedium::construct_color(
//         box1,
//         0.01,
//         &Color3::construct(&[0.0, 0.0, 0.0]),
//     )));
//     objects.add(Arc::new(ConstantMedium::construct_color(
//         box2,
//         0.01,
//         &Color3::construct(&[1.0, 1.0, 1.0]),
//     )));

//     objects
// }

// pub fn final_scene() -> HittableList {
//     let mut boxes1 = HittableList::new();
//     let ground = Arc::new(Lambertian::construct(&Color3::construct(&[
//         0.48, 0.83, 0.53,
//     ])));

//     const BOXES_PER_SIDE: u32 = 20;
//     for i in 0..BOXES_PER_SIDE {
//         for j in 0..BOXES_PER_SIDE {
//             let w = 100.0;
//             let x0 = -1000.0 + i as f64 * w;
//             let z0 = -1000.0 + j as f64 * w;
//             let y0 = 0.0;
//             let x1 = x0 + w;
//             let y1 = random_double_range(1.0, 101.0);
//             let z1 = z0 + w;

//             boxes1.add(Arc::new(Box::construct(
//                 &Point3::construct(&[x0, y0, z0]),
//                 &Point3::construct(&[x1, y1, z1]),
//                 ground.clone(),
//             )));
//         }
//     }

//     let mut objects = HittableList::new();
//     objects.add(Arc::new(BVHNode::construct2(&boxes1, 0.0, 1.0)));

//     let light = Arc::new(DiffuseLight::construct_color(&Color3::construct(&[
//         7.0, 7.0, 7.0,
//     ])));
//     objects.add(Arc::new(XzRect::construct(
//         123.0, 423.0, 147.0, 412.0, 554.0, light,
//     )));

//     let center1 = Point3::construct(&[400.0, 400.0, 200.0]);
//     let center2 = center1 + Vec3::construct(&[30.0, 0.0, 0.0]);
//     let moving_sphere_mat = Arc::new(Lambertian::construct(&Color3::construct(&[0.7, 0.3, 0.1])));
//     objects.add(Arc::new(MovingSphere::construct(
//         &center1,
//         &center2,
//         0.0,
//         1.0,
//         50.0,
//         moving_sphere_mat,
//     )));

//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[260.0, 150.0, 45.0]),
//         50.0,
//         Arc::new(Dielectric::construct(1.5)),
//     )));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 150.0, 145.0]),
//         50.0,
//         Arc::new(Metal::construct(&Color3::construct(&[0.8, 0.8, 0.9]), 1.0)),
//     )));

//     let boundary = Arc::new(Sphere::construct(
//         &Point3::construct(&[360.0, 150.0, 145.0]),
//         70.0,
//         Arc::new(Dielectric::construct(1.5)),
//     ));
//     objects.add(boundary.clone());
//     objects.add(Arc::new(ConstantMedium::construct_color(
//         boundary,
//         0.2,
//         &Color3::construct(&[0.2, 0.4, 0.9]),
//     )));
//     let boundary = Arc::new(Sphere::construct(
//         &Point3::construct(&[0.0, 0.0, 0.0]),
//         5000.0,
//         Arc::new(Dielectric::construct(1.5)),
//     ));
//     objects.add(Arc::new(ConstantMedium::construct_color(
//         boundary,
//         0.0001,
//         &Color3::construct(&[1.0, 1.0, 1.0]),
//     )));

//     let img = image::open("earthmap.jpg").expect("Failed to open image");
//     let width: u32 = img.width();
//     let height: u32 = img.height();
//     let mut data: Vec<u8> = Vec::new();
//     for (_x, _y, pixel) in img.pixels() {
//         let rgba = pixel.0;
//         let (r, g, b) = (rgba[0], rgba[1], rgba[2]);
//         data.push(r);
//         data.push(g);
//         data.push(b);
//     }
//     let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::construct(&data, width, height));
//     let earth_surface = Arc::new(Lambertian::construct_texture(earth_texture));
//     let globe = Arc::new(Sphere::construct(
//         &Point3::construct(&[400.0, 200.0, 400.0]),
//         100.0,
//         earth_surface,
//     ));
//     objects.add(globe);
//     let pertext = Arc::new(NoiseTexture::construct(0.1));
//     objects.add(Arc::new(Sphere::construct(
//         &Point3::construct(&[220.0, 280.0, 300.0]),
//         80.0,
//         Arc::new(Lambertian::construct_texture(pertext)),
//     )));

//     let mut boxes2 = HittableList::new();
//     let white: Arc<dyn Material> = Arc::new(Lambertian::construct(&Color3::construct(&[
//         0.73, 0.73, 0.73,
//     ])));
//     let ns: u32 = 1000;
//     for _j in 0..ns {
//         boxes2.add(Arc::new(Sphere::construct(
//             &Point3::random_range(0.0, 165.0),
//             10.0,
//             white.clone(),
//         )));
//     }

//     objects.add(Arc::new(Translate::construct(
//         Arc::new(RotateY::construct(
//             Arc::new(BVHNode::construct2(&boxes2, 0.0, 1.0)),
//             15.0,
//         )),
//         &Vec3::construct(&[-100.0, 270.0, 395.0]),
//     )));

//     objects
// }

fn main() {
    // let img =

    let path = std::path::Path::new("output/book3/image7.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: i32 = 50;

    // World
    // let mut world = random_scene();

    let world: HittableList = cornell_box();
    let lights: Arc<dyn Hittable> = Arc::new(XzRect::construct(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffuseLight::construct_color(&Color3::construct(&[
            15.0, 15.0, 15.0,
        ]))),
    ));

    let lookfrom = Point3::construct(&[278.0, 278.0, -800.0]);
    let lookat = Point3::construct(&[278.0, 278.0, 0.0]);
    let vfov = 40.0;
    let aperture = 0.0;
    let background = Color3::construct(&[0.0, 0.0, 0.0]);
    // let mth = 0;
    // match mth {
    //     1 => {
    //         world = random_scene();
    //         aperture = 0.1;
    //     }
    //     _ => {
    //         world = final_scene();
    //         // background = Color3::construct(&[0.0, 0.0, 0.0]);
    //     }
    // }

    // Camera
    // let lookfrom: Point3 = Point3::construct(&[13.0, 2.0, 3.0]);
    // let lookat: Point3 = Point3::construct(&[0.0, 0.0, 0.0]);
    let vup: Vec3 = Vec3::construct(&[0.0, 1.0, 0.0]);
    let dist_to_focus: f64 = 10.0;

    let cam: Camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        &[vfov, ASPECT_RATIO, aperture, dist_to_focus],
        0.0,
        1.0,
    );

    // Render
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64)
    };

    let thread_num: u32 = 10;
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT - j - 1);
            let mut pixel_color: Color3 = Color3::construct(&[0.0, 0.0, 0.0]);

            let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
            let mut recv: Vec<mpsc::Receiver<Color3>> = Vec::new();
            for _s in 0..thread_num {
                let (tx, rx) = mpsc::channel();
                recv.push(rx);
                let cam = cam.clone();
                let world = world.clone();
                let max_depth = MAX_DEPTH;
                let image_width = IMAGE_WIDTH;
                let image_height = IMAGE_HEIGHT;
                let i_f64 = i as f64;
                let j_f64 = j as f64;
                let lights = lights.clone();

                let handle = thread::spawn(move || {
                    for _t in 0..(SAMPLES_PER_PIXEL / thread_num) {
                        let u: f64 = (i_f64 + random_double()) / (image_width - 1) as f64;
                        let v: f64 = (j_f64 + random_double()) / (image_height - 1) as f64;
                        let r: Ray = cam.get_ray(u, v);
                        tx.send(ray_color(
                            &r,
                            &background,
                            &world,
                            lights.clone(),
                            max_depth,
                        ))
                        .unwrap();
                    }
                });
                handles.push(handle);
            }
            let mut cnt = 0u32;
            // 第一层遍历：遍历接收者
            for rec in recv {
                // 第二层遍历：遍历接收者收到的数据
                for received in rec {
                    pixel_color += received;
                    cnt += 1;
                }
            }
            for thread in handles {
                thread.join().unwrap();
            }
            assert_eq!(cnt, SAMPLES_PER_PIXEL);
            let rgb: [u8; 3] = write_color(&pixel_color, SAMPLES_PER_PIXEL);
            *pixel = image::Rgb(rgb);
            progress.inc(1);
        }
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputing image fails.").red()),
    }

    exit(0);
}
