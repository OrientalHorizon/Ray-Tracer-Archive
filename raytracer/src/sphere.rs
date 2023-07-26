use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::rt_weekend::PI;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere<T: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: Material> Sphere<T> {
    // pub fn new() -> Self {
    //     Self {
    //         center: Point3::new(),
    //         radius: 0.0,
    //     }
    // }
    pub fn construct(center: &Point3, radius: f64, mat_ptr: T) -> Self {
        Self {
            center: *center,
            radius,
            mat_ptr,
        }
    }
    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta: f64 = (-p.y()).acos();
        let phi: f64 = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl<'a, T: Material> Hittable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let det: f64 = half_b * half_b - a * c;
        if det < 0.0 {
            return None;
        }
        let sqrtd: f64 = det.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec: HitRecord = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::<T>::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = Some(&self.mat_ptr);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let radius = self.radius;
        Some(Aabb::construct(
            &(self.center - Vec3::construct(&[radius, radius, radius])),
            &(self.center + Vec3::construct(&[radius, radius, radius])),
        ))
    }
}
