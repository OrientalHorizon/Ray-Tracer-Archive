use crate::hittable::{HitRecord, Hittable};
use crate::rt_weekend::random_int;
use crate::{aabb::Aabb, hittable_list::HittableList};
use std::sync::Arc;

pub struct BVHNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub box_: Aabb,
} // bounding volume hierachy

impl BVHNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::construct(&list.objects, time0, time1)
    }
    pub fn construct(objects: &Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        // println!("{}", objects.len());
        let mut src_objects = objects.clone();
        let left;
        let right;
        let axis = random_int(0, 2) as usize;
        let obj_len = objects.len();

        if obj_len == 0 {
            panic!("BVH: Empty list");
        }

        if obj_len == 1 {
            left = Some(src_objects.remove(0));
            // pop, 往左移
            right = None;
        } else {
            src_objects.sort_unstable_by(|a, b| {
                let mut aabb_a = Aabb::new();
                let mut aabb_b = Aabb::new();
                let bool_a = a.bounding_box(time0, time1, &mut aabb_a);
                let bool_b = b.bounding_box(time0, time1, &mut aabb_b);
                if bool_a == false || bool_b == false {
                    panic!("fuck you");
                }
                let min_a = aabb_a.minimum().e[axis];
                let min_b = aabb_b.minimum().e[axis];
                f64::partial_cmp(&min_a, &min_b).unwrap()
            });
            if obj_len == 2 {
                right = Some(src_objects.remove(1));
                left = Some(src_objects.remove(0));
            } else {
                let objects2 = src_objects.split_off(obj_len / 2);
                left = Some(Arc::new(Self::construct(&src_objects, time0, time1)));
                right = Some(Arc::new(Self::construct(&objects2, time0, time1)));
            }
        }
        let mut box_left = Aabb::new();
        left.as_ref()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left);
        let box_ = if right.is_some() {
            let mut box_right = Aabb::new();
            right
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right);
            Aabb::surrounding_box(&box_left, &box_right)
        } else {
            box_left
        };
        Self { left, right, box_ }
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.box_;
        true
    }
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.box_.hit(r, t_min, t_max) {
            return false;
        }
        // println!("BVH: hit");
        let mut t_max_mut = t_max;
        *rec = HitRecord::new();
        let mut tmp_rec = HitRecord::new();
        let mut ret = false;
        if self
            .left
            .as_ref()
            .unwrap()
            .hit(r, t_min, t_max, &mut tmp_rec)
        {
            t_max_mut = tmp_rec.t;
            *rec = tmp_rec.clone();
            ret = true;
        }
        if self.right.is_some() {
            if self
                .right
                .as_ref()
                .unwrap()
                .hit(r, t_min, t_max_mut, &mut tmp_rec)
            {
                *rec = tmp_rec;
                ret = true;
            }
        }
        ret
    }
}
