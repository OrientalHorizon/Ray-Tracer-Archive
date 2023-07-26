use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::{cross, dot, Point3, Vec3};
use std::sync::Arc;

pub struct Triangle {
    pub poi: Point3,
    pub unit_normal: Vec3,
    pub edge1: Vec3,
    pub edge2: Vec3,
    pub det: f64,
    pub mat_ptr: Arc<dyn Material>,
    pub bbox: Aabb,
}

impl Triangle {
    pub fn construct(points: &[Point3], mat: Arc<dyn Material>) -> Self {
        let edge1: Vec3 = points[1] - points[0];
        let edge2: Vec3 = points[2] - points[0];
        let unit_normal: Vec3 = cross(&edge1, &edge2);
        let det = dot(&unit_normal, &unit_normal);
        let mut mn: Point3 = Point3::new();
        let mut mx: Point3 = Point3::new();
        for i in 0..3 {
            mn.e[i] = points[0].e[i].min(points[1].e[i].min(points[2].e[i]));
            mx.e[i] = points[0].e[i].max(points[1].e[i].max(points[2].e[i]));
        }
        Self {
            poi: points[0],
            unit_normal,
            edge1,
            edge2,
            det,
            mat_ptr: Arc::clone(&mat),
            bbox: Aabb::construct(&mn, &mx),
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let proj1 = dot(&(self.poi - r.origin()), &(self.unit_normal));
        let proj2 = dot(&r.direction(), &(self.unit_normal));
        if proj2 == 0.0 {
            return false;
        }
        let t = proj1 / proj2;
        if t < t_min || t > t_max {
            return false;
        }
        let hit_point = r.at(t);
        let ap: Vec3 = hit_point - self.poi;
        // 用向量表示 ap，看它在不在三角形内
        let perp_ab = cross(&self.edge1, &self.unit_normal) / self.det;
        let perp_ac = cross(&self.unit_normal, &self.edge2) / self.det;
        let c1 = dot(&ap, &perp_ab);
        let c2 = dot(&ap, &perp_ac);
        if c1 < 0.0 || c2 < 0.0 || c1 + c2 > 1.0 {
            return false;
        }
        rec.t = t;
        rec.p = hit_point;
        rec.mat_ptr = Some(Arc::clone(&self.mat_ptr));
        rec.u = c1;
        rec.v = c2;
        rec.set_face_normal(r, &self.unit_normal);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}
