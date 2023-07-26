use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rt_weekend::{random_double, INFINITY};
use crate::texture::SolidColor;
use crate::vec3::Color3;

pub struct ConstantMedium<T1: Hittable, T2: Material> {
    pub boundary: T1,
    pub phase_function: T2,
    pub neg_inv_density: f64,
}
impl<T1: Hittable> ConstantMedium<T1, Isotropic<SolidColor>> {
    // pub fn construct(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: Arc::clone(&b),
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Arc::new(Isotropic::construct(a)),
    //     }
    // }
    pub fn construct_color(b: T1, d: f64, c: &Color3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::construct_color(c),
        }
    }
}
impl<T1: Hittable, T2: Material> Hittable for ConstantMedium<T1, T2> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(r, -INFINITY, INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);

        if rec2.is_none() {
            return None;
        }
        let rec2 = rec2.unwrap();

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::new();
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Color3::construct(&[1.0, 0.0, 0.0]); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = Some(&self.phase_function);

        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
