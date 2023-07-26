use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::vec::Vec;

pub struct BVHNode<'a> {
    left: &'a dyn Hittable,
    right: &'a dyn Hittable,
    aabb: Aabb,
}

pub fn box_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable, axis: u32) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in BVHNode constructor.");
    }
    let mut box_a = box_a.unwrap();
    let mut box_b = box_b.unwrap();
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
pub fn box_x_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable) -> Ordering {
    box_compare(a, b, 2)
}

impl<'a> BVHNode<'a> {
    pub fn new(left: &'a dyn Hittable, right: &'a dyn Hittable, aabb: &Aabb) -> Self {
        Self {
            left,
            right,
            aabb: *aabb,
        }
    }
    pub fn construct2(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::construct(&list.objects, 0, list.objects.len() as u32, time0, time1)
    }
    pub fn construct(
        src_objects: &[&'a dyn Hittable],
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> Self {
        let start = start as usize;
        let end = end as usize;
        let mut objects: Vec<&'a dyn Hittable> = src_objects.to_vec();
        let axis = rand::random::<u32>() % 3u32;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span: usize = end - start;
        let left: &'a dyn Hittable;
        let right: &'a dyn Hittable;
        if object_span == 1 {
            left = objects[start];
            right = objects[start];
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
            left = &BVHNode::construct(&objects, start as u32, mid as u32, time0, time1);
            right = &BVHNode::construct(&objects, mid as u32, end as u32, time0, time1);
        }
        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        if box_left.is_none() || box_right.is_none() {
            eprintln!("No bounding box in BVHNode constructor.");
        }
        let mut box_left = box_left.unwrap();
        let mut box_right = box_right.unwrap();
        let aabb = Aabb::surrounding_box(&box_left, &box_right);
        Self::new(left, right, &aabb)
    }
}

impl<'a> Hittable for BVHNode<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }
        // let mut rec = HitRecord::new();
        let hit_left = self.left.hit(r, t_min, t_max);

        let hit_right = self.right.hit(
            r,
            t_min,
            if hit_left.is_some() {
                hit_left.t
            } else {
                t_max
            },
        );
        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.aabb)
    }
}
