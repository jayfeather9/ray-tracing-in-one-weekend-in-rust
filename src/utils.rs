pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub fn random_double_in(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_double()
}
