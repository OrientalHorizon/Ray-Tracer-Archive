use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::onb::Onb;
use crate::pdf::random_to_sphere;
use crate::ray::Ray;
use crate::rt_weekend::{INFINITY, PI};
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    // pub fn new() -> Self {
    //     Self {
    //         center: Point3::new(),
    //         radius: 0.0,
    //     }
    // }
    pub fn construct(center: &Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
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
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = Some(Arc::clone(&self.mat_ptr));
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let radius = self.radius;
        *output_box = Aabb::construct(
            &(self.center - Vec3::construct(&[radius, radius, radius])),
            &(self.center + Vec3::construct(&[radius, radius, radius])),
        );
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(&Ray::construct(o, v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }
        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - *o).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let distance_sq = direction.length_squared();
        let uvw = Onb::build_from_w(&direction);
        uvw.local(&random_to_sphere(self.radius, distance_sq))
    }
}
