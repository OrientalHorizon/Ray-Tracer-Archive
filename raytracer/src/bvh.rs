use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;
use std::vec::Vec;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: Aabb,
}

pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: u32) -> Ordering {
    let mut box_a = Aabb::new();
    let mut box_b = Aabb::new();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVHNode constructor.");
    }
    match axis {
        0 => {
            if box_a.minimum().x() < box_b.minimum().x() {
                Ordering::Less
            } else if box_a.minimum().x() < box_b.minimum().x() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }

        1 => {
            if box_a.minimum().y() < box_b.minimum().y() {
                Ordering::Less
            } else if box_a.minimum().y() < box_b.minimum().y() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }

        2 => {
            if box_a.minimum().z() < box_b.minimum().z() {
                Ordering::Less
            } else if box_a.minimum().z() < box_b.minimum().z() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }

        _ => panic!("Invalid axis"),
    }
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl BVHNode {
    pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, aabb: &Aabb) -> Self {
        Self {
            left: Arc::clone(&left),
            right: Arc::clone(&right),
            aabb: *aabb,
        }
    }
    pub fn construct2(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::construct(&list.objects, 0, list.objects.len() as u32, time0, time1)
    }
    pub fn construct(
        src_objects: &[Arc<dyn Hittable>],
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> Self {
        let start = start as usize;
        let end = end as usize;
        let mut objects: Vec<Arc<dyn Hittable>> = src_objects.to_vec();
        let axis = rand::random::<u32>() % 3u32;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span: usize = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            if comparator(&objects[start as usize], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_by(comparator);
            let mid = start + object_span / 2;
            left = Arc::new(BVHNode::construct(
                &objects,
                start as u32,
                mid as u32,
                time0,
                time1,
            ));
            right = Arc::new(BVHNode::construct(
                &objects, mid as u32, end as u32, time0, time1,
            ));
        }
        let mut box_left = Aabb::new();
        let mut box_right = Aabb::new();
        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in BVHNode constructor.");
        }
        let aabb = Aabb::surrounding_box(&box_left, &box_right);
        Self::new(left, right, &aabb)
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
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.aabb;
        true
    }
}
