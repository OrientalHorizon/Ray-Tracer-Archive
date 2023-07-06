use crate::vec3::Color3;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    // pub fn new() -> Self {
    //     Self {
    //         origin: Point3::new(),
    //         direction: Vec3::new(),
    //     }
    // }
    pub fn construct(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }
    // pub fn at(&self, t: f64) -> Point3 {
    //     self.origin + self.direction * t
    // }
    // pub fn origin(&self) -> Point3 {
    //     self.origin
    // }
    // pub fn direction(&self) -> Vec3 {
    //     self.direction
    // }
    pub fn ray_color(&self) -> Color3 {
        let unit_direction = self.direction.unit();
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);
        Color3::construct(&[1.0, 1.0, 1.0]) * (1.0 - t) + Color3::construct(&[0.5, 0.7, 1.0]) * t
    }
}
