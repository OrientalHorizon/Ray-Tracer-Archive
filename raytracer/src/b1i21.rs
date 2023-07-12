use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rt_weekend;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
// use material::Lambertian;
use ray::Ray;
use rt_weekend::{random_double, random_double_range};
use sphere::Sphere;
use vec3::{Color3, Point3, Vec3};

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

pub fn ray_color(r: &Ray, world: &mut dyn Hittable, depth: i32) -> Color3 {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Color3::new();
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::new();
        let mut attenuation: Color3 = Color3::new();
        if rec
            .mat_ptr
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color3::construct(&[0.0, 0.0, 0.0]);
    }

    let unit_direction = r.direction.unit();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Color3::construct(&[1.0, 1.0, 1.0]) * (1.0 - t) + Color3::construct(&[0.5, 0.7, 1.0]) * t
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::construct(&Color3::construct(&[0.5, 0.5, 0.5])));
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::construct(&[
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            ]);

            if (center - Point3::construct(&[4.0, 0.2, 0.0])).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color3::random() * Color3::random();
                    Rc::new(Lambertian::construct(&albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Rc::new(Metal::construct(&albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::construct(1.5))
                };
                world.add(Rc::new(Sphere::construct(&center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::construct(1.5));
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[0.0, 1.0, 0.0]),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::construct(&Color3::construct(&[0.4, 0.2, 0.1])));
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[-4.0, 1.0, 0.0]),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::construct(&Color3::construct(&[0.7, 0.6, 0.5]), 0.0));
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[4.0, 1.0, 0.0]),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let path = std::path::Path::new("output/book1/image21.jpg");
    // 青天蓝日满地绿
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth: i32 = 50;

    // World
    let mut world = random_scene();

    // Camera
    let lookfrom: Point3 = Point3::construct(&[13.0, 2.0, 3.0]);
    let lookat: Point3 = Point3::construct(&[0.0, 0.0, 0.0]);
    let vup: Vec3 = Vec3::construct(&[0.0, 1.0, 0.0]);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let cam: Camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);
            let mut pixel_color: Color3 = Color3::construct(&[0.0, 0.0, 0.0]);
            for _s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v: f64 = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world, max_depth);
            }

            let rgb: [u8; 3] = write_color(&pixel_color, samples_per_pixel);
            *pixel = image::Rgb(rgb);
        }
        progress.inc(1);
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
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}