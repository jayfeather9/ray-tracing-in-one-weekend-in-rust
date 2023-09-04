pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: 0.0,
    max: -1.0,
};
pub const FULL_INTERVAL: Interval = Interval {
    min: std::f64::NEG_INFINITY,
    max: std::f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn full() -> Self {
        Self {
            min: std::f64::NEG_INFINITY,
            max: std::f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
