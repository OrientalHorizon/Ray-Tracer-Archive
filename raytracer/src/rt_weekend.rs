const infinity: f64 = std::f64::INFINITY;
const pi: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}
