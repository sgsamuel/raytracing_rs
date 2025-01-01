use std::fmt;
use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64
}

impl Ray {
    pub const ZERO: Ray = Ray {
        orig: Point3::ZERO,
        dir: Vec3::ZERO,
        tm: 0.0
    };

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { orig: origin, dir: direction, tm: 0.0}
    }

    pub fn with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self { orig: origin, dir: direction, tm: time}
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {}, Direction: {}", self.orig, self.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_and_direction() {
        let orig: Point3 = Point3::new(3.0, 2.0, 1.0);
        let dir: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let ray: Ray = Ray::new(orig, dir);
        assert_eq!(ray.origin(), &orig);
        assert_eq!(ray.direction(), &dir);
    }

    #[test]
    fn at() {
        let orig: Point3 = Point3::new(3.0, 2.0, 1.0);
        let dir: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let ray: Ray = Ray::new(orig, dir);

        assert_eq!(ray.at(0.0), Point3::new(3.0, 2.0, 1.0));
        assert_eq!(ray.at(0.5), Point3::new(3.5, 3.0, 2.5));
        assert_eq!(ray.at(1.0), Point3::new(4.0, 4.0, 4.0));

        
        assert_eq!(ray.at(2.0), Point3::new(5.0, 6.0, 7.0));
    }
}