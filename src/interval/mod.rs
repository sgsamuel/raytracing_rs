use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: -f64::INFINITY
    };

    pub const UNIVERSE: Interval = Interval {
        min: -f64::INFINITY,
        max: f64::INFINITY
    };

    pub const UNIT: Interval = Interval {
        min: 0.0,
        max: 1.0
    };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_interval(a: &Interval, b: &Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max)
        }
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[inline]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
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

    pub fn expand(&mut self, delta: f64) {
        let padding: f64 = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }
}


impl<'ivl> Add<f64> for &'ivl Interval {
    type Output = Interval;

    fn add(self, other: f64) -> Interval {
        Interval { min: self.min + other, max: self.max + other}
    }
}

#[allow(clippy::op_ref)]
impl Add<f64> for Interval {    
    type Output = Interval;

    #[inline]
    fn add(self, other: f64) -> Interval {
        &self + other
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    #[inline]
    fn add(self, other: Interval) -> Interval {
        other + self
    }
}

impl<'ivl> Add<&'ivl Interval> for f64 {
    type Output = Interval;

    #[inline]
    fn add(self, other: &'ivl Interval) -> Interval {
        *other + self
    }
}


#[cfg(test)]
mod tests {
    use crate::interval::*;

    #[test]
    fn size() {
        let ivl: Interval = Interval::new(0.0, 1.0);
        assert_eq!(ivl.size(), 1.0);
    }

    #[test]
    fn contains() {
        let ivl: Interval = Interval::new(0.0, 1.0);
        assert_eq!(ivl.contains(-1.0), false);
        assert_eq!(ivl.contains(0.0), true);
        assert_eq!(ivl.contains(0.5), true);
        assert_eq!(ivl.contains(1.0), true);
        assert_eq!(ivl.contains(2.0), false);
    }

    #[test]
    fn surrounds() {
        let ivl: Interval = Interval::new(0.0, 1.0);
        assert_eq!(ivl.surrounds(-1.0), false);
        assert_eq!(ivl.surrounds(0.0), false);
        assert_eq!(ivl.surrounds(0.5), true);
        assert_eq!(ivl.surrounds(1.0), false);
        assert_eq!(ivl.surrounds(2.0), false);
    }

    #[test]
    fn clamp() {
        let ivl: Interval = Interval::new(0.0, 1.0);
        assert_eq!(ivl.clamp(-1.0), 0.0);
        assert_eq!(ivl.clamp(0.0), 0.0);
        assert_eq!(ivl.clamp(0.5), 0.5);
        assert_eq!(ivl.clamp(1.0), 1.0);
        assert_eq!(ivl.clamp(2.0), 1.0);
    }

    #[test]
    fn expand() {
        let orig_min: f64 = 0.0;
        let orig_max: f64 = 1.0;
        let mut ivl: Interval = Interval::new(orig_min, orig_max);

        let mut delta: f64 = 0.0;
        ivl.expand(delta);
        assert_eq!(ivl.min, orig_min);
        assert_eq!(ivl.max, orig_max);

        delta = 1.0;
        ivl.expand(delta);
        assert_eq!(ivl.min, orig_min - delta / 2.0);
        assert_eq!(ivl.max, orig_max +  delta / 2.0);
    }
}