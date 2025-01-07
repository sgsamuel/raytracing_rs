use std::ops::Add;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Axis, Point3f, Vec3f};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY
    };

    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE
    };

    pub fn from_interval(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb: Self = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_point(a: &Point3f, b: &Point3f) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        let mut aabb: Self = Self {
            x: Interval::new(
                f64::min(a.component(Axis::X), b.component(Axis::X)),
                f64::max(a.component(Axis::X), b.component(Axis::X))
            ),
            y: Interval::new(
                f64::min(a.component(Axis::Y), b.component(Axis::Y)),
                f64::max(a.component(Axis::Y), b.component(Axis::Y))
            ),
            z: Interval::new(
                f64::min(a.component(Axis::Z), b.component(Axis::Z)),
                f64::max(a.component(Axis::Z), b.component(Axis::Z))
            )
        };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_bounding_box(box1: &AABB, box2: &AABB) -> Self {
        let mut aabb: Self = Self {
            x: Interval::from_interval(&box1.x, &box2.x),
            y: Interval::from_interval(&box1.y, &box2.y),
            z: Interval::from_interval(&box1.z, &box2.z)
        };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn longest_axis(&self) -> Axis {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() >= self.y.size() && self.x.size() >= self.z.size() {
            Axis::X
        } 
        else if self.y.size() >= self.z.size() {
            Axis::Y
        } 
        else {
            Axis::Z
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let ray_orig: &Point3f = ray.origin();
        let ray_dir: &Vec3f  = ray.direction();

        for &axis in Axis::iterator() {
            let ax_ivl: Interval = self.axis_interval(axis);
            let ad_inv: f64 = 1.0 / ray_dir.component(axis);

            let t0 = (ax_ivl.min - ray_orig.component(axis)) * ad_inv;
            let t1 = (ax_ivl.max - ray_orig.component(axis)) * ad_inv;
 
            let min_check: f64 = f64::max(ray_t.min, f64::min(t0, t1));
            let max_check: f64 = f64::min(ray_t.max, f64::max(t0, t1));

            if max_check <= min_check {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta: f64 = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }
}

impl<'v, 'bb> Add<&'v Vec3f> for &'bb AABB {
    type Output = AABB;

    fn add(self, other: &'v Vec3f) -> AABB {
        AABB { 
            x: self.x + other.component(Axis::X), 
            y: self.y + other.component(Axis::Y), 
            z: self.z + other.component(Axis::Z) 
        }
    }
}

impl Add<Vec3f> for AABB {    
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: Vec3f) -> AABB {
        &self + &other
    }
}

impl<'v> Add<&'v Vec3f> for AABB {    
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: &'v Vec3f) -> AABB {
        &self + other
    }
}

impl<'bb> Add<Vec3f> for &'bb AABB {    
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: Vec3f) -> AABB {
        self + &other
    }
}

impl<'bb, 'v> Add<&'bb AABB> for &'v Vec3f {
    type Output = AABB;

    #[inline]
    fn add(self, other: &'bb AABB) -> AABB {
        other + self
    }
}

impl Add<AABB> for Vec3f {
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: AABB) -> AABB {
        &other + &self
    }
}

impl<'bb> Add<&'bb AABB> for Vec3f {
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: &'bb AABB) -> AABB {
        other + &self
    }
}

impl<'v> Add<AABB> for &'v Vec3f {
    type Output = AABB;

    #[inline]
    #[allow(clippy::op_ref)]
    fn add(self, other: AABB) -> AABB {
        &other + self
    }
}
