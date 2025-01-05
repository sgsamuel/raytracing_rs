use std::fmt;
use crate::vec3::{Point3f, Vec3f};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    orig: Point3f,
    dir: Vec3f,
    tm: f64
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {}, Direction: {}", self.orig, self.dir)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::ZERO
    }
}

impl Ray {
    pub const ZERO: Ray = Ray {
        orig: Point3f::ZERO,
        dir: Vec3f::ZERO,
        tm: 0.0
    };

    pub fn new(origin: &Point3f, direction: &Vec3f) -> Self {
        Self { orig: *origin, dir: *direction, tm: 0.0}
    }

    pub fn with_time(origin: &Point3f, direction: &Vec3f, time: f64) -> Self {
        Self { orig: *origin, dir: *direction, tm: time}
    }

    pub fn origin(&self) -> &Point3f {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3f {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3f {
        self.orig + self.dir * t
    }
}


#[cfg(test)]
mod tests {
    use crate::ray::*;

    #[test]
    fn origin_direction_time() {
        let orig: Point3f = Point3f::new(3.0, 2.0, 1.0);
        let dir: Vec3f = Vec3f::new(1.0, 2.0, 3.0);
        let ray1: Ray = Ray::new(&orig, &dir);

        assert_eq!(ray1.origin(), &orig);
        assert_eq!(ray1.direction(), &dir);
        assert_eq!(ray1.time(), 0.0);

        let tm: f64 = 0.5;
        let ray2: Ray = Ray::with_time(&orig, &dir, tm);
        assert_eq!(ray2.origin(), &orig);
        assert_eq!(ray2.direction(), &dir);
        assert_eq!(ray2.time(), tm);
    }

    #[test]
    fn at() {
        let orig: Point3f = Point3f::new(3.0, 2.0, 1.0);
        let dir: Vec3f = Vec3f::new(1.0, 2.0, 3.0);
        let ray: Ray = Ray::new(&orig, &dir);

        assert_eq!(ray.at(0.0), Point3f::new(3.0, 2.0, 1.0));
        assert_eq!(ray.at(0.5), Point3f::new(3.5, 3.0, 2.5));
        assert_eq!(ray.at(1.0), Point3f::new(4.0, 4.0, 4.0));

        
        assert_eq!(ray.at(2.0), Point3f::new(5.0, 6.0, 7.0));
    }
}