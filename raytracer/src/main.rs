use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod rt_weekend;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use rt_weekend::random_double;
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

pub fn ray_color(r: &Ray, world: &mut dyn Hittable) -> Color3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color3::construct(&[1.0, 1.0, 1.0]));
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
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component.
    [
        (256.0 * rt_weekend::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * rt_weekend::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * rt_weekend::clamp(b, 0.0, 0.999)) as u8,
    ]
}

fn main() {
    let path = std::path::Path::new("output/book1/image6.jpg");
    // 青天蓝日满地绿
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[0.0, 0.0, -1.0]),
        0.5,
    )));
    world.add(Rc::new(Sphere::construct(
        &Point3::construct(&[0.0, -100.5, -1.0]),
        100.0,
    )));

    // Camera
    let cam: Camera = Camera::new();

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
                pixel_color += ray_color(&r, &mut world);
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
