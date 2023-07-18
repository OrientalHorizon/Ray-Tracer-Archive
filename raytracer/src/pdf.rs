use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rt_weekend::PI;
use crate::vec3::{dot, random_cosine_direction, Point3, Vec3};
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    pub uvw: Onb,
}
impl CosinePdf {
    pub fn construct(w: &Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = dot(&direction.unit(), &self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    pub o: Point3,
    pub ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn construct(p: Arc<dyn Hittable>, origin: &Point3) -> Self {
        Self {
            o: *origin,
            ptr: Arc::clone(&p),
        }
    }
}
impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}