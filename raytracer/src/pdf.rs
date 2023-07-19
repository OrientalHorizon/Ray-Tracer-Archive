use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rt_weekend::{random_double, PI};
use crate::vec3::{dot, random_cosine_direction, Point3, Vec3};
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Debug)]
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

pub struct MixturePdf {
    pub p: [Arc<dyn Pdf>; 2],
}
impl MixturePdf {
    pub fn construct(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self {
            p: [Arc::clone(&p0), Arc::clone(&p1)],
        }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
