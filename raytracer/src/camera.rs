use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        Self {
            origin: Point3::construct(&[0.0, 0.0, 0.0]),
            horizontal: Vec3::construct(&[viewport_width, 0.0, 0.0]),
            vertical: Vec3::construct(&[0.0, viewport_height, 0.0]),
            lower_left_corner: Point3::construct(&[
                -viewport_width / 2.0,
                -viewport_height / 2.0,
                -focal_length,
            ]),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::construct(
            &self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
        )
    }
}