use std::fmt;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Axis, Point3f, Vec3f};

#[derive(Clone)]
pub struct Quad {
    quad_start: Point3f,
    u: Vec3f,
    v: Vec3f,
    w: Vec3f,
    quad_eq_d: f64,
    normal: Vec3f,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q: {}; u: {}; v: {}; Material: {}", self.quad_start, self.u, self.v, self.mat)
    }
}

impl Quad {
    pub fn new(quad_start: &Point3f, u: &Vec3f, v: &Vec3f, mat: Arc<dyn Material>) -> Self {
        let n: Vec3f = Vec3f::cross(u, v);
        let normal: &Vec3f = &Vec3f::unit_vector(&n);
        let w: Vec3f = n / Vec3f::length_squared(&n);
        let quad_eq_d: f64 = Vec3f::dot(normal, quad_start);

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

    #[inline]
    pub fn new_box(a: &Point3f, b: &Point3f, mat: Arc<dyn Material>) -> Arc<HittableList>{
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
        let mut sides: HittableList = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min: Point3f = Point3f::new(
            f64::min(a.component(Axis::X), b.component(Axis::X)),
            f64::min(a.component(Axis::Y), b.component(Axis::Y)),
            f64::min(a.component(Axis::Z), b.component(Axis::Z))
        );
        let max: Point3f = Point3f::new(
            f64::max(a.component(Axis::X), b.component(Axis::X)),
            f64::max(a.component(Axis::Y), b.component(Axis::Y)),
            f64::max(a.component(Axis::Z), b.component(Axis::Z))
        );

        let dx: Vec3f = Vec3f::new(max.component(Axis::X) - min.component(Axis::X), 0.0, 0.0);
        let dy: Vec3f = Vec3f::new(0.0, max.component(Axis::Y) - min.component(Axis::Y), 0.0);
        let dz: Vec3f = Vec3f::new(0.0, 0.0, max.component(Axis::Z) - min.component(Axis::Z));

        sides.add(Arc::new(Quad::new(
                &Point3f::new(min.component(Axis::X), min.component(Axis::Y), max.component(Axis::Z)),
                &dx,
                &dy,
                mat.clone()
            ))
        ); // front
        sides.add(Arc::new(Quad::new(
                &Point3f::new(max.component(Axis::X), min.component(Axis::Y), max.component(Axis::Z)),
                &-dz,
                &dy,
                mat.clone()
            ))
        ); // right
        sides.add(Arc::new(Quad::new(
                &Point3f::new(max.component(Axis::X), min.component(Axis::Y), min.component(Axis::Z)),
                &-dx,
                &dy,
                mat.clone()
            ))
        ); // back
        sides.add(Arc::new(Quad::new(
                &Point3f::new(min.component(Axis::X), min.component(Axis::Y), min.component(Axis::Z)),
                &dz,
                &dy,
                mat.clone()
            ))
        ); // left
        sides.add(Arc::new(Quad::new(
            &Point3f::new(min.component(Axis::X), max.component(Axis::Y), max.component(Axis::Z)),
            &dx,
            &-dz,
            mat.clone()
            ))
        ); // top
        sides.add(Arc::new(Quad::new(
                &Point3f::new(min.component(Axis::X), min.component(Axis::Y), min.component(Axis::Z)),
                &dx,
                &dz,
                mat.clone()
            ))
        ); // bottom

        Arc::new(sides)
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
        let denominator: f64 = Vec3f::dot(&self.normal, ray.direction());

        // No hit if the ray is parallel to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t: f64 = (self.quad_eq_d - Vec3f::dot(&self.normal, ray.origin())) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection: Vec3f = ray.at(t);
        let planar_hitpt_vector: Vec3f = intersection - self.quad_start;
        let alpha: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&planar_hitpt_vector, &self.v));
        let beta: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&self.u, &planar_hitpt_vector));

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