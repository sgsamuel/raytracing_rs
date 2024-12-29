use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn with_origin_and_direction(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    // Getter for direction
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    // Method to calculate the point at parameter t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}