use crate::vec3::{cross, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Onb {
    pub axis: [Vec3; 3],
}
impl std::ops::Index<usize> for Onb {
    type Output = Vec3;
    fn index(&self, i: usize) -> &Self::Output {
        &self.axis[i]
    }
}
impl std::ops::IndexMut<usize> for Onb {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.axis[i]
    }
}
impl Onb {
    pub fn build_from_w(n: &Vec3) -> Self {
        let mut axes = [Vec3::new(); 3];
        axes[2] = n.unit();
        let a = if (axes[2].x()).abs() > 0.9 {
            Vec3::construct(&[0.0, 1.0, 0.0])
        } else {
            Vec3::construct(&[1.0, 0.0, 0.0])
        };
        axes[1] = cross(&axes[2], &a).unit();
        axes[0] = cross(&axes[2], &axes[1]);
        Self { axis: axes }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
    pub fn local_f64(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
}
