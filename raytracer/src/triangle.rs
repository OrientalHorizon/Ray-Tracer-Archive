use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::{cross, dot, Point3, Vec3};
use std::sync::Arc;

// unsafe {
//     static mut TRIANGLE_COUNT: usize = 0;
// }

pub struct Triangle {
    pub poi: Point3,
    pub unit_normal: Vec3,
    pub ab: Vec3,
    pub ac: Vec3,
    pub perp_ab: Vec3,
    pub perp_ac: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub bbox: Aabb,
    // pub tex_a: (f64, f64),
    // pub tex_ab: (f64, f64),
    // pub tex_ac: (f64, f64),
}

impl Triangle {
    pub fn construct(
        points: &[Point3],
        mat: Arc<dyn Material>,
        // (ua, va): (f64, f64),
        // (ub, vb): (f64, f64),
        // (uc, vc): (f64, f64),
    ) -> Self {
        let (a, b, c) = (points[0], points[1], points[2]);
        let ab: Vec3 = points[1] - points[0];
        let ac: Vec3 = points[2] - points[0];
        let normal: Vec3 = cross(&ab, &ac);
        let det = normal.length();
        let mut mn: Point3 = Point3::new();
        let mut mx: Point3 = Point3::new();
        for i in 0..3 {
            mn.e[i] = f64::min(f64::min(a.e[i], b.e[i]), c.e[i]) - 0.0001;
            mx.e[i] = f64::max(f64::max(a.e[i], b.e[i]), c.e[i]) + 0.0001;
        }
        let unit_normal = normal.unit();
        Self {
            poi: points[0],
            unit_normal,
            ab,
            ac,
            perp_ab: cross(&unit_normal, &ab) / det,
            perp_ac: cross(&ac, &unit_normal) / det,
            mat_ptr: Arc::clone(&mat),
            bbox: Aabb::construct(&mn, &mx),
            // tex_a: (ua, va),
            // tex_ab: (ub - ua, vb - va),
            // tex_ac: (uc - ua, vc - va),
        }
    }
    // pub fn get_pixel_uv(&self, u: f64, v: f64) -> (f64, f64) {
    //     let (ua, va) = self.tex_a;
    //     let (uab, vab) = self.tex_ab;
    //     let (uac, vac) = self.tex_ac;
    //     let ub = ua + uab * u + uac * v;
    //     let vb = va + vab * u + vac * v;
    //     (ub, vb)
    // }
}

impl Hittable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let op = self.poi - r.origin();
        let t = dot(&op, &self.unit_normal) / dot(&r.direction(), &self.unit_normal);
        if t < t_min || t > t_max {
            // println!("None");
            return false;
        }
        let hit_point = r.at(t);
        let ap: Vec3 = hit_point - self.poi;
        // 用向量表示 ap，看它在不在三角形内
        let c1 = dot(&ap, &self.perp_ac);
        let c2 = dot(&ap, &self.perp_ab);
        if c1 >= 0.0 && c2 >= 0.0 && c1 + c2 <= 1.0 {
            rec.t = t;
            rec.p = hit_point;
            rec.mat_ptr = Some(Arc::clone(&self.mat_ptr));
            // let (u, v) = self.get_pixel_uv(c1, c2);
            rec.u = c1;
            rec.v = c2;
            rec.set_face_normal(&r, &self.unit_normal);
            return true;
        }
        // println!("None");
        false
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}
