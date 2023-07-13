use crate::rt_weekend::random_int;
use crate::vec3::{dot, Point3, Vec3};
use std::vec::Vec;

pub struct Perlin {
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<u32>,
    pub perm_y: Vec<u32>,
    pub perm_z: Vec<u32>,
}
impl Perlin {
    const POINT_COUNT: u32 = 256;
    pub fn new() -> Self {
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(Self::POINT_COUNT as usize);
        for _i in 0..Self::POINT_COUNT {
            ranvec.push(Vec3::random_range(-1.0, 1.0).unit());
        }
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u: f64 = p.x() - p.x().floor();
        let v: f64 = p.y() - p.y().floor();
        let w: f64 = p.z() - p.z().floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i: i32 = p.x().floor() as i32;
        let j: i32 = p.y().floor() as i32;
        let k: i32 = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
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
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in c.iter().enumerate() {
            for j in c[i.0].iter().enumerate() {
                for k in c[i.0][j.0].iter().enumerate() {
                    let weight_v =
                        Vec3::construct(&[u - i.0 as f64, v - j.0 as f64, w - k.0 as f64]);
                    accum += (i.0 as f64 * uu + (1.0 - i.0 as f64) * (1.0 - uu))
                        * (j.0 as f64 * vv + (1.0 - j.0 as f64) * (1.0 - vv))
                        * (k.0 as f64 * ww + (1.0 - k.0 as f64) * (1.0 - ww))
                        * dot(&c[i.0][j.0][k.0], &weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Point3) -> f64 {
        let depth: u32 = 7;
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
