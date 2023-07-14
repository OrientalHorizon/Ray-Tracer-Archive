use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rt_weekend::{random_double, INFINITY};
use crate::vec3::Color3;
use std::rc::Rc;

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f64,
}
impl ConstantMedium {
    // pub fn construct(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: Rc::clone(&b),
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Rc::new(Isotropic::construct(a)),
    //     }
    // }
    pub fn construct_color(b: Rc<dyn Hittable>, d: f64, c: &Color3) -> Self {
        Self {
            boundary: Rc::clone(&b),
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::construct_color(c)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Color3::construct(&[1.0, 0.0, 0.0]); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = Some(Rc::clone(&self.phase_function));

        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
