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
        let i = ((4.0 * p.x()).floor() as i32) & 255i32;
        let j = ((4.0 * p.y()).floor() as i32) & 255i32;
        let k = ((4.0 * p.z()).floor() as i32) & 255i32;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
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
}
