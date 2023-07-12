use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new() -> Self {
        Self {
            minimum: Point3::new(),
            maximum: Point3::new(),
        }
    }
    pub fn construct(minimum: &Point3, maximum: &Point3) -> Self {
        Self {
            minimum: *minimum,
            maximum: *maximum,
        }
    }

    pub fn minimum(&self) -> Point3 {
        self.minimum
    }
    pub fn maximum(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            // let t0 = min((minimum[a] - r.origin()[a]) / r.direction()[a],
            //              (maximum[a] - r.origin()[a]) / r.direction()[a]);
            // let t1 = max((minimum[a] - r.origin()[a]) / r.direction()[a],
            //              (maximum[a] - r.origin()[a]) / r.direction()[a]);
            // t_min = max(t0, t_min);
            // t_max = min(t1, t_max);
            // if t_max <= t_min {
            //     return false;
            // }
            let inv_d: f64 = 1.0 / r.direction().e[a];
            let mut t0: f64 = (self.minimum().e[a] - r.origin().e[a]) * inv_d;
            let mut t1: f64 = (self.maximum().e[a] - r.origin().e[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min: f64 = if t_min > t0 { t_min } else { t0 };
            let t_max: f64 = if t_max < t1 { t_max } else { t1 };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small: Point3 = Point3::construct(&[
            box0.minimum().x().min(box1.minimum().x()),
            box0.minimum().y().min(box1.minimum().y()),
            box0.minimum().z().min(box1.minimum().z()),
        ]);
        let big: Point3 = Point3::construct(&[
            box0.maximum().x().max(box1.maximum().x()),
            box0.maximum().y().max(box1.maximum().y()),
            box0.maximum().z().max(box1.maximum().z()),
        ]);
        Aabb::construct(&small, &big)
    }
}
