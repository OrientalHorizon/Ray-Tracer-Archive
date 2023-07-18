use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::rt_weekend::{random_double_range, INFINITY};
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;

pub struct XyRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}
impl XyRect {
    pub fn construct(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: Arc::clone(&mat),
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}
impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x: f64 = r.origin().x() + t * r.direction().x();
        let y: f64 = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::construct(&[0.0, 0.0, 1.0]));
        rec.mat_ptr = Some(Arc::clone(&self.mp));
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        // non-zero width in each dimension
        *output_box = Aabb::construct(
            &Point3::construct(&[self.x0, self.y0, self.k - 0.0001]),
            &Point3::construct(&[self.x1, self.y1, self.k + 0.0001]),
        );
        true
    }
}

pub struct XzRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl XzRect {
    pub fn construct(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: Arc::clone(&mat),
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}
impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let x: f64 = r.origin().x() + t * r.direction().x();
        let z: f64 = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::construct(&[0.0, 1.0, 0.0]));
        rec.mat_ptr = Some(Arc::clone(&self.mp));
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        // non-zero width in each dimension
        *output_box = Aabb::construct(
            &Point3::construct(&[self.x0, self.k - 0.0001, self.z0]),
            &Point3::construct(&[self.x1, self.k + 0.0001, self.z1]),
        );
        true
    }
    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(&Ray::construct(origin, v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(&v, &rec.normal) / v.length()).abs();
        distance_squared / cosine / area
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Point3::construct(&[
            random_double_range(self.x0, self.x1),
            self.k,
            random_double_range(self.z0, self.z1),
        ]);
        random_point - *origin
    }
}

pub struct YzRect {
    pub mp: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl YzRect {
    pub fn construct(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: Arc::clone(&mat),
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}
impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t: f64 = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let y: f64 = r.origin().y() + t * r.direction().y();
        let z: f64 = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::construct(&[1.0, 0.0, 0.0]));
        rec.mat_ptr = Some(Arc::clone(&self.mp));
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        // non-zero width in each dimension
        *output_box = Aabb::construct(
            &Point3::construct(&[self.k - 0.0001, self.y0, self.z0]),
            &Point3::construct(&[self.k + 0.0001, self.y1, self.z1]),
        );
        true
    }
}
