// use crate::vec3::Color3;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            direction: Vec3::new(),
            tm: 0.0,
        }
    }
    pub fn construct(origin: &Point3, direction: &Vec3, tm: f64) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
            tm,
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
}
