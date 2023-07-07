use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

mod ray;
mod vec3;

use ray::Ray;
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

pub fn ray_color(r: &Ray) -> Color3 {
    let res = hit_sphere(&Point3::construct(&[0.0, 0.0, -1.0]), &0.5, r);
    if res > 0.0 {
        let n: Vec3 = (r.at(res) - Vec3::construct(&[0.0, 0.0, -1.0])).unit();
        return 0.5 * Color3::construct(&[n.x() + 1.0, n.y() + 1.0, n.z() + 1.0]);
    }
    let unit_direction = r.direction.unit();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Color3::construct(&[1.0, 1.0, 1.0]) * (1.0 - t) + Color3::construct(&[0.5, 0.7, 1.0]) * t
}

fn main() {
    let path = std::path::Path::new("output/book1/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::construct(&[0.0, 0.0, 0.0]);
    let horizontal: Vec3 = Vec3::construct(&[viewport_width, 0.0, 0.0]);
    let vertical: Vec3 = Vec3::construct(&[0.0, viewport_height, 0.0]);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::construct(&[0.0, 0.0, focal_length]);

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
            let u: f64 = (i as f64) / ((image_width - 1) as f64);
            let v: f64 = (j as f64) / ((image_height - 1) as f64);
            let r: Ray = Ray::construct(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin),
            );
            let pixel_color: Color3 = ray_color(&r);
            let rgb: [u8; 3] = pixel_color.rgb();
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
