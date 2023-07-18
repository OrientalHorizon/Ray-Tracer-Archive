use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::rt_weekend::{degrees_to_radians, INFINITY};
use crate::vec3::Point3;
use crate::vec3::{dot, Vec3};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            mat_ptr: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
    // pub fn construct(p: &Point3, normal: &Vec3, t: f64, front_face: bool) -> Self {
    //     Self {
    //         p: *p,
    //         normal: *normal,
    //         t,
    //         front_face,
    //     }
    // }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::construct(&[1.0, 0.0, 0.0])
    }
}

pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}
impl Translate {
    pub fn construct(p: Arc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            ptr: Arc::clone(&p),
            offset: *displacement,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::construct(&(r.origin() - self.offset), &r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        let norm = rec.normal;
        rec.set_face_normal(&moved_r, &norm);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(_time0, _time1, output_box) {
            return false;
        }
        *output_box = Aabb::construct(
            &(output_box.minimum() + self.offset),
            &(output_box.maximum() + self.offset),
        );
        true
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: Aabb,
}
impl RotateY {
    pub fn construct(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians: f64 = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::new();
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut mini = Point3::construct(&[INFINITY, INFINITY, INFINITY]);
        let mut maxi = Point3::construct(&[-INFINITY, -INFINITY, -INFINITY]);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum().x() + (1.0 - i as f64) * bbox.minimum().x();
                    let y = j as f64 * bbox.maximum().y() + (1.0 - j as f64) * bbox.minimum().y();
                    let z = k as f64 * bbox.maximum().z() + (1.0 - k as f64) * bbox.minimum().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::construct(&[newx, y, newz]);

                    for c in 0..3 {
                        mini.e[c] = mini.e[c].min(tester.e[c]);
                        maxi.e[c] = maxi.e[c].max(tester.e[c]);
                    }
                }
            }
        }
        bbox = Aabb::construct(&mini, &maxi);
        Self {
            ptr: Arc::clone(&p),
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.e[0] = self.cos_theta * r.origin().e[0] - self.sin_theta * r.origin().e[2];
        origin.e[2] = self.sin_theta * r.origin().e[0] + self.cos_theta * r.origin().e[2];

        direction.e[0] = self.cos_theta * r.direction().e[0] - self.sin_theta * r.direction().e[2];
        direction.e[2] = self.sin_theta * r.direction().e[0] + self.cos_theta * r.direction().e[2];

        let rotated_r = Ray::construct(&origin, &direction, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.e[0] = self.cos_theta * rec.p.e[0] + self.sin_theta * rec.p.e[2];
        p.e[2] = -self.sin_theta * rec.p.e[0] + self.cos_theta * rec.p.e[2];

        normal.e[0] = self.cos_theta * rec.normal.e[0] + self.sin_theta * rec.normal.e[2];
        normal.e[2] = -self.sin_theta * rec.normal.e[0] + self.cos_theta * rec.normal.e[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}

pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}
impl FlipFace {
    pub fn construct(p: Arc<dyn Hittable>) -> Self {
        Self {
            ptr: Arc::clone(&p),
        }
    }
}
impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.ptr.hit(r, t_min, t_max, rec) {
            return false;
        }
        rec.front_face = !rec.front_face;
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }
}
