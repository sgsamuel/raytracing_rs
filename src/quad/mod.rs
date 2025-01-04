use std::fmt;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Quad {
    quad_start: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    quad_eq_d: f64,
    normal: Vec3,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q: {}; u: {}; v: {}; Material: {}", self.quad_start, self.u, self.v, self.mat)
    }
}

impl Quad {
    pub fn new(quad_start: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let n: Vec3 = Vec3::cross(u, v);
        let normal: &Vec3 = &Vec3::unit_vector(&n);
        let w: Vec3 = n / Vec3::length_squared(&n);
        let quad_eq_d: f64 = Vec3::dot(normal, quad_start);

        // Compute the bounding box of all four vertices.
        let diagonal1: AABB = AABB::from_point(quad_start, &(quad_start + u + v));
        let diagonal2: AABB = AABB::from_point(&(quad_start + u), &(quad_start + v));
        let bounding_box = AABB::from_bounding_box(&diagonal1, &diagonal2);
        Self { 
            quad_start: *quad_start, 
            u: *u,
            v: *v,
            w,
            quad_eq_d, 
            normal: *normal, 
            mat, 
            bounding_box 
        }
    }

    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if !Interval::UNIT.contains(a) || !Interval::UNIT.contains(b) {
            return None;
        }
        Some((a, b))
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denominator: f64 = Vec3::dot(&self.normal, ray.direction());

        // No hit if the ray is parallel to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t: f64 = (self.quad_eq_d - Vec3::dot(&self.normal, ray.origin())) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection: Vec3 = ray.at(t);
        let planar_hitpt_vector: Vec3 = intersection - self.quad_start;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        match Self::is_interior(alpha, beta) {
            Some(uv) => {
                // Ray hits the 2D shape; set the hit record
                let rec: HitRecord = HitRecord::new(
                    intersection, 
                    self.mat.clone(), 
                    t, 
                    uv, 
                    ray, 
                    &self.normal
                );
                
                Some(rec)
            },
            None => None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}