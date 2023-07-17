use crate::perlin::Perlin;
use crate::rt_weekend::clamp;
use crate::vec3::{Color3, Point3};
use std::sync::Arc;

pub trait Texture: Send + Sync {
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
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}
impl CheckerTexture {
    // pub fn construct(ev: Arc<dyn Texture>, od: Arc<dyn Texture>) -> Self {
    //     Self {
    //         odd: Arc::clone(&od),
    //         even: Arc::clone(&ev),
    //     }
    // }
    pub fn construct_color(c1: &Color3, c2: &Color3) -> Self {
        Self {
            even: Arc::new(SolidColor::construct(c1)),
            odd: Arc::new(SolidColor::construct(c2)),
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
        Color3::construct(&[1.0, 1.0, 1.0])
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p)).sin())
    }
}

pub struct ImageTexture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_scanline: u32,
}
impl ImageTexture {
    pub const BYTES_PER_PIXEL: u32 = 3;

    pub fn construct(data: &[u8], width: u32, height: u32) -> Self {
        Self {
            data: data.to_owned(),
            width,
            height,
            bytes_per_scanline: width * Self::BYTES_PER_PIXEL,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color3 {
        if self.data.is_empty() {
            return Color3::construct(&[0.0, 1.0, 1.0]);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i: u32 = (u * self.width as f64) as u32;
        let mut j: u32 = (v * self.height as f64) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale: f64 = 1.0 / 255.0;
        let pixel_index: usize = (j * self.bytes_per_scanline + i * Self::BYTES_PER_PIXEL) as usize;
        Color3::construct(&[
            color_scale * self.data[pixel_index] as f64,
            color_scale * self.data[pixel_index + 1] as f64,
            color_scale * self.data[pixel_index + 2] as f64,
        ])
    }
}
