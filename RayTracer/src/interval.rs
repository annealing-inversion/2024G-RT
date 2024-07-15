use crate::vec3::Vec3;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

// pub const Interval::empty: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
// pub const Interval::universe: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };


impl Interval {
    pub const empty: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const universe: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn new_from_intervals(a: &Interval, b: &Interval) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, t: f64) -> bool {
        t >= self.min && t <= self.max
    }
    pub fn surrounds(&self, t: f64) -> bool {
        t > self.min && t < self.max
    }
    pub fn clamp(&self, x:f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta * 0.5;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, discriminant:f64) -> Interval {
        Interval { min: self.min + discriminant, max: self.max + discriminant }
    }
}