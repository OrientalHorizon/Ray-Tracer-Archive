// use crate::vec3::Point3;
use crate::hittable;
use crate::ray::Ray;

use hittable::{HitRecord, Hittable};
use std::rc::Rc;
use std::vec::Vec;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    // pub fn construct(object: Rc<dyn Hittable>) -> Self {
    //     Self {
    //         objects: {
    //             let mut vec = Vec::new();
    //             vec.push(object);
    //             vec
    //         },
    //     }
    // }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}