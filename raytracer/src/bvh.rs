use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;
use std::vec::Vec;

struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    aabb: AABB,
}

pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: u32) -> bool {
    let mut box_a = AABB::new();
    let mut box_b = AABB::new();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVHNode constructor.");
    }
    match axis {
        0 => box_a.min().x() < box_b.min().x(),
        1 => box_a.min().y() < box_b.min().y(),
        _ => box_a.min().z() < box_b.min().z(),
    }
}
pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 2)
}

impl BVHNode {
    pub fn new(left: Rc<dyn Hittable>, right: Rc<dyn Hittable>, aabb: &AABB) -> Self {
        Self {
            left: Rc::clone(&left),
            right: Rc::clone(&right),
            aabb: *aabb,
        }
    }
    pub fn construct(
        src_objects: &mut Vec<Rc<dyn Hittable>>,
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects.clone();
        let axis = rand::random::<u32>() % 3u32;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span: u32 = end - start;
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b));
            let mid = start + object_span / 2;
            left = Rc::new(BVHNode::construct(&mut objects, start, mid, time0, time1));
            right = Rc::new(BVHNode::construct(&mut objects, mid, end, time0, time1));
        }
        let mut box_left = AABB::new();
        let mut box_right = AABB::new();
        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in BVHNode constructor.");
        }
        let aabb = AABB::surrounding_box(&box_left, &box_right);
        Self::new(left, right, aabb)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.aabb;
        true
    }
}
