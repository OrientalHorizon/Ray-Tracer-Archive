use crate::perlin::Perlin;
use crate::vec3::{Color3, Point3};
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3;
}

pub struct SolidColor {
    pub color_value: Color3,
}

impl SolidColor {
    // pub fn new() -> Self {
    //     Self {
    //         color_value: Color3::new(),
    //     }
    // }
    pub fn construct(color_value: &Color3) -> Self {
        Self {
            color_value: *color_value,
        }
    }
    // pub fn construct_3(red: f64, green: f64, blue: f64) -> Self {
    //     Self {
    //         color_value: Color3::construct(&[red, green, blue]),
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>,
}
impl CheckerTexture {
    // pub fn construct(ev: Rc<dyn Texture>, od: Rc<dyn Texture>) -> Self {
    //     Self {
    //         odd: Rc::clone(&od),
    //         even: Rc::clone(&ev),
    //     }
    // }
    pub fn construct_color(c1: &Color3, c2: &Color3) -> Self {
        Self {
            even: Rc::new(SolidColor::construct(c1)),
            odd: Rc::new(SolidColor::construct(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        let sines: f64 = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl NoiseTexture {
    // pub fn new() -> Self {
    //     Self {
    //         noise: Perlin::new(),
    //         scale: 1.0,
    //     }
    // }
    pub fn construct(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color3 {
        Color3::construct(&[1.0, 1.0, 1.0]) * self.noise.noise(&(self.scale * *p))
    }
}
