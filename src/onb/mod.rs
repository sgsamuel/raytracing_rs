use std::fmt;
use std::slice::Iter;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::vec3::{Axis, Vec3f};


#[derive(Clone, Copy, Debug)]
pub enum BasisAxis {
    U,
    V,
    W,
}

impl BasisAxis {
    pub fn iterator() -> Iter<'static, BasisAxis> {
        static BASISAXES: [BasisAxis; 3] = [BasisAxis::U, BasisAxis::V, BasisAxis::W];
        BASISAXES.iter()
    }
}

impl Distribution<BasisAxis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BasisAxis {
        match rng.gen_range(0..=2) {
            0 => BasisAxis::U,
            1 => BasisAxis::V,
            _ => BasisAxis::W,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ONB {
    u: Vec3f,
    v: Vec3f,
    w: Vec3f
}

impl fmt::Display for ONB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.u, self.v, self.w)
    }
}

impl Default for ONB {
    fn default() -> Self {
        ONB::STANDARD
    }
}

impl ONB {
    pub const STANDARD: ONB = ONB {
        u: Vec3f::E1, 
        v: Vec3f::E2, 
        w: Vec3f::E3
    };

    pub fn new(n: &Vec3f) -> Self {
        let w: Vec3f = Vec3f::unit_vector(n);
        let a: Vec3f = if w.component(Axis::X).abs() > 0.9 {
            Vec3f::E2
        }
        else {
            Vec3f::E1
        };

        let v: Vec3f = Vec3f::unit_vector(&Vec3f::cross(&w, &a));
        let u: Vec3f = Vec3f::cross(&w, &v);
        Self { u, v, w }
    }

    pub fn component(&self, axis: BasisAxis) -> &Vec3f {
        match axis {
            BasisAxis::U => &self.u,
            BasisAxis::V => &self.v,
            BasisAxis::W => &self.w,
        }
    }

    pub fn transform(&self, v: &Vec3f) -> Vec3f {
        // Transform from basis coordinates to local space.
        v.component(Axis::X) * self.u + v.component(Axis::Y) * self.v + v.component(Axis::Z) * self.w
    }
}