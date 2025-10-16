use std::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn default() -> Self {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval {
            min,
            max
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, other: f64) -> bool{
        self.min <= other && other <= self.max
    }

    pub fn surrounds(&self, other: f64) -> bool {
        self.min < other && other < self.max
    }

    pub fn empty() -> Interval {
        Interval{
            min: f64::INFINITY,
            max: -f64::INFINITY
        }
    }

    pub fn universe() -> Interval {
        Interval{
            min: -f64::INFINITY,
            max: f64::INFINITY
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    } 
}