use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub const ZERO: Ray = Ray {
        orig: Point3::ZERO,
        dir: Vec3::ZERO,
    };

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}