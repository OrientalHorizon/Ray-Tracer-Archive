use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    pub fn construct(t: &[f64]) -> Self {
        Self { e: [t[0], t[1], t[2]] }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }
    pub fn unit(&self) -> Self {
        *self / self.length()
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.e[0] * other.e[0] +
            self.e[1] * other.e[1] +
            self.e[2] * other.e[2]
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (255.999 * self.e[0]) as u8,
            (255.999 * self.e[1]) as u8,
            (255.999 * self.e[2]) as u8,
        ]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    // 内积
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    // 数乘
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other,
            ]
        }
    }
}

impl Mul<Vec3> for f64 {
    // 数乘
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                other.e[0] * self,
                other.e[1] * self,
                other.e[2] * self,
            ]
        }
    }
}

impl Div<f64> for Vec3 {
    // 数除
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            e: [
                self.e[0] / other,
                self.e[1] / other,
                self.e[2] / other,
            ]
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ]
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;
