use crate::ray::Ray;
use crate::rt_weekend::{degrees_to_radians, random_double_range};
use crate::vec3::{cross, random_in_unit_disk, Point3, Vec3};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
    // Shutter open / close time
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        scope: &[f64],
        // vfov, aspect_ratio, aperture, focus_dist
        time0: f64,
        time1: f64,
    ) -> Self {
        let vfov = scope[0];
        let aspect_ratio = scope[1];
        let aperture = scope[2];
        let focus_dist = scope[3];
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let scope: Vec3 = *lookfrom - *lookat;
        let w: Vec3 = scope.unit();
        let u: Vec3 = cross(vup, &w).unit();
        let v: Vec3 = cross(&w, &u);

        let origin = *lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray::construct(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset),
            random_double_range(self.time0, self.time1),
        )
    }
}
