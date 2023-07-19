use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;

#[derive(Debug)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn construct(
        center0: &Point3,
        center1: &Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: *center0,
            center1: *center1,
            time0,
            time1,
            radius,
            mat_ptr: Arc::clone(&m),
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center(r.time());
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let det: f64 = half_b * half_b - a * c;
        if det < 0.0 {
            return false;
        }
        let sqrtd: f64 = det.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(Arc::clone(&self.mat_ptr));
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        let box0 = crate::aabb::Aabb::construct(
            &(self.center(time0) - Vec3::construct(&[self.radius, self.radius, self.radius])),
            &(self.center(time0) + Vec3::construct(&[self.radius, self.radius, self.radius])),
        );
        let box1 = crate::aabb::Aabb::construct(
            &(self.center(time1) - Vec3::construct(&[self.radius, self.radius, self.radius])),
            &(self.center(time1) + Vec3::construct(&[self.radius, self.radius, self.radius])),
        );
        *output_box = Aabb::surrounding_box(&box0, &box1);
        true
    }
}
