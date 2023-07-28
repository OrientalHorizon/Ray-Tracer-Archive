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

    // texture coord.
    pub uv_a: (f64, f64),
    pub uv_ab: (f64, f64),
    pub uv_ac: (f64, f64),
}

impl Triangle {
    pub fn new(
        a: &Point3,
        b: &Point3,
        c: &Point3,
        mat: Arc<dyn Material>,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
    ) -> Self {
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
            uv_a: (ua, va),
            uv_ab: (ub - ua, vb - va),
            uv_ac: (uc - ua, vc - va),
        }
    }
    pub fn to_texture_coord(&self, u0: f64, v0: f64) -> (f64, f64) {
        let u = self.uv_a.0 + self.uv_ab.0 * u0 + self.uv_ac.0 * v0;
        let v = self.uv_a.1 + self.uv_ab.1 * u0 + self.uv_ac.1 * v0;
        (u, v)
    }
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
        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            let (x, y) = self.to_texture_coord(u, v);
            *rec = HitRecord {
                p,
                normal: self.n,
                t,
                u: x,
                v: y,
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
