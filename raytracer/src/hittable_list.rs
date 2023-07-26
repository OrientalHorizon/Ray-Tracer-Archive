// use crate::vec3::Point3;
use crate::aabb::Aabb;
use crate::hittable;
use crate::ray::Ray;
use std::sync::Arc;

use hittable::{HitRecord, Hittable};
use std::vec::Vec;

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn construct(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: {
                let vec = vec![object];
                vec
            },
        }
    }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut rec = HitRecord::new();
        for object in self.objects.iter() {
            let temp_rec = object.hit(r, t_min, closest_so_far);
            if temp_rec.is_some() {
                let temp_rec = temp_rec.unwrap();
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec.clone();
            }
        }
        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = Aabb::new();
        let mut first_box: bool = true;

        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                Some(output_box) => {
                    temp_box = if first_box {
                        output_box
                    } else {
                        Aabb::surrounding_box(&temp_box, &output_box)
                    }
                }
                None => return None,
            }
            first_box = false;
        }
        Some(temp_box)
    }
}
