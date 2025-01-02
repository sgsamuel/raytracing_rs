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

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
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

#[cfg(test)]
mod tests {
    use super::*;

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