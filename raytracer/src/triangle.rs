use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
// use crate::rt_weekend::{random_double, random_double_range};
use crate::vec3::*;
// use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone)]
pub struct Triangle {
    pub a: Point3,
    pub n: Vec3,
    pub pb: Vec3,
    pub pc: Vec3,
    //pc perpendicular to ac with length of ac/2*area
    pub bbox: Aabb,
    pub mat: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: &Point3, b: &Point3, c: &Point3, mat: Arc<dyn Material>) -> Self {
        let ab = *b - *a;
        let ac = *c - *a;
        let normal = cross(&ab, &ac);
        let area2 = normal.length();
        let n = normal.unit();
        let mut min = Point3::default();
        let mut max = Point3::default();
        for i in 0..3 {
            min.e[i] = a.e[i].min(b.e[i]).min(c.e[i]) - 0.0001;
            max.e[i] = a.e[i].max(b.e[i]).max(c.e[i]) + 0.0001;
        }
        Self {
            a: *a,
            n,
            pb: cross(&n, &ab) / area2,
            pc: cross(&ac, &n) / area2,
            mat,
            bbox: Aabb::construct(&min, &max),
        }
    }

    // pub fn area(&self) -> f64 {
    //     cross(&self.pb, &self.pc).length() / 2.0
    // }

    // pub fn get_edges(&self) -> (Vec3, Vec3) {
    //     let area2 = self.area() * 2.0;
    //     let ab = cross(&self.pb, &self.n) * area2;
    //     let ac = cross(&self.n, &self.pc) * area2;

    //     let normal = cross(&ab, &ac);
    //     if normal.unit() == self.n {
    //         (ab, ac)
    //     } else {
    //         panic!("triangle get edges error")
    //     }
    // }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oa = self.a - r.origin();
        let t = dot(&oa, &self.n) / dot(&r.direction(), &self.n);
        if t < t_min || t_max < t {
            return false;
        }
        let p = r.at(t);
        let ap = p - self.a;
        let u = dot(&ap, &self.pc);
        let v = dot(&ap, &self.pb);
        // AP = uAB + vAC
        if u >= 0. && v >= 0. && u + v <= 1. {
            *rec = HitRecord {
                p,
                normal: self.n,
                t,
                u,
                v,
                front_face: true, //set it true if you want to emit light!!!
                mat_ptr: Some(Arc::clone(&self.mat)),
            };
            // rec.set_face_normal(r, &self.n);
            true
        } else {
            false
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}
