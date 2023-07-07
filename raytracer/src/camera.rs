use crate::ray::Ray;
use crate::rt_weekend::degrees_to_radians;
use crate::vec3::{cross, Point3, Vec3};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let scope: Vec3 = *lookfrom - *lookat;
        let w: Vec3 = scope.unit();
        let u: Vec3 = cross(vup, &w).unit();
        let v: Vec3 = cross(&w, &u);

        let origin = *lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::construct(
            &self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
        )
    }
}
