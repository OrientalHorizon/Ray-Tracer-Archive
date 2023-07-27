use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use std::sync::Arc;

pub struct Box_ {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}
impl Box_ {
    pub fn construct(p0: &Point3, p1: &Point3, ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XyRect::construct(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        )));
        sides.add(Arc::new(XyRect::construct(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        )));

        sides.add(Arc::new(XzRect::construct(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        )));
        sides.add(Arc::new(XzRect::construct(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        )));

        sides.add(Arc::new(YzRect::construct(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        )));
        sides.add(Arc::new(YzRect::construct(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr.clone(),
        )));
        Self {
            box_min: *p0,
            box_max: *p1,
            sides,
        }
    }
}
impl Hittable for Box_ {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::construct(&self.box_min, &self.box_max);
        true
    }
}
