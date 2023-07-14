use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Color3;
use std::rc::Rc;

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn construct(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Material>) -> Self {
        Self {
            boundary: Rc::clone(&b),
            neg_inv_density: -1.0 / d,
            phase_function: Rc::clone(&a),
        }
    }
    // pub fn construct_color(b: Rc<dyn Hittable>, d: f64, c: &Color3) -> Self {
    //     Self {
    //         boundary: Rc::clone(&b),
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Rc::new()
    //     }
}
