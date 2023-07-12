use crate::rt_weekend::{random_double, random_int};
use crate::vec3::Point3;
use std::vec::Vec;

pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<u32>,
    pub perm_y: Vec<u32>,
    pub perm_z: Vec<u32>,
}
impl Perlin {
    const POINT_COUNT: u32 = 256;
    pub fn new() -> Self {
        let mut ranfloat: Vec<f64> = Vec::with_capacity(Self::POINT_COUNT as usize);
        for _i in 0..Self::POINT_COUNT {
            ranfloat.push(random_double());
        }
        Self {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u: f64 = p.x() - p.x().floor();
        let v: f64 = p.y() - p.y().floor();
        let w: f64 = p.z() - p.z().floor();

        let i: i32 = p.x().floor() as i32;
        let j: i32 = p.y().floor() as i32;
        let k: i32 = p.z().floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[(self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }
    fn perlin_generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = Vec::with_capacity(Self::POINT_COUNT as usize);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }
    fn permute(p: &mut [u32], n: u32) {
        for i in (1..n).rev() {
            let target = random_int(0, i);
            p.swap(i as usize, target as usize);
        }
    }
    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in c.iter().enumerate() {
            for j in c[i.0].iter().enumerate() {
                for k in c[i.0][j.0].iter().enumerate() {
                    accum += (i.0 as f64 * u + (1.0 - i.0 as f64) * (1.0 - u))
                        * (j.0 as f64 * v + (1.0 - j.0 as f64) * (1.0 - v))
                        * (k.0 as f64 * w + (1.0 - k.0 as f64) * (1.0 - w))
                        * c[i.0][j.0][k.0];
                }
            }
        }
        accum
    }
}
