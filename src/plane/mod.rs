use std::fmt;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utilities::random;
use crate::vec3::{Axis, Point3f, Vec3f};


pub trait Interior {
    fn is_interior(plane_coord: (f64, f64)) -> Option<(f64, f64)>;
}

#[derive(Clone)]
pub struct Plane {
    orig: Point3f,
    dir_a: Vec3f,
    dir_b: Vec3f,
    w: Vec3f,
    area: f64,
    quad_eq_d: f64,
    normal: Vec3f,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {}; Direction A: {}; Direction B: {}; Material: {}", self.orig, self.dir_a, self.dir_b, self.mat)
    }
}

impl Plane {
    pub fn new(orig: &Point3f, dir_a: &Vec3f, dir_b: &Vec3f, mat: Arc<dyn Material>) -> Self {
        let n: Vec3f = Vec3f::cross(dir_a, dir_b);
        let normal: &Vec3f = &Vec3f::unit_vector(&n);
        let w: Vec3f = n / Vec3f::length_squared(&n);
        let area: f64 = n.length();
        let quad_eq_d: f64 = Vec3f::dot(normal, orig);

        let bounding_box = AABB::UNIVERSE;

        Self { 
            orig: *orig, 
            dir_a: *dir_a,
            dir_b: *dir_b,
            w,
            area,
            quad_eq_d, 
            normal: *normal,
            mat,
            bounding_box
        }
    }

    pub fn planar_hit_coordinates(&self, intersection: &Vec3f) -> (f64, f64) {
        let planar_hitpt_vector: Vec3f = intersection - self.orig;
        let alpha: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&planar_hitpt_vector, &self.dir_b));
        let beta: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&self.dir_a, &planar_hitpt_vector));
        (alpha, beta)
    }
}

impl Hittable for Plane {
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

        // Determine planar intersection
        let intersection: Vec3f = ray.at(t);

        // Ray always hits plane
        let rec: HitRecord = HitRecord::new(
            intersection, 
            self.mat.clone(), 
            t, 
            self.planar_hit_coordinates(&intersection), 
            ray, 
            &self.normal
        );

        return Some(rec);
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    fn pdf_value(&self, origin: &Point3f, direction: &Vec3f) -> f64 {
        if let Some(rec) =  self.hit(&Ray::new(origin, direction), &Interval::new(0.001, f64::INFINITY)) {
            let distance_squared: f64 = rec.t * rec.t * direction.length_squared();
            let cos_theta: f64 = f64::abs(Vec3f::dot(direction, &rec.normal) / direction.length());
            return distance_squared / (cos_theta * self.area);
        }
        0.0
    }

    fn random(&self, origin: &Point3f) -> Vec3f {
        let p: Vec3f = self.orig + (random() * self.dir_a) + (random() * self.dir_b);
        p - *origin
    }
}


#[derive(Clone)]
pub struct Quad {
    plane: Plane,
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane: {}", self.plane)
    }
}

impl Quad {
    pub fn new(orig: &Point3f, dir_a: &Vec3f, dir_b: &Vec3f, mat: Arc<dyn Material>) -> Self {
        let mut plane: Plane = Plane::new(orig, dir_a, dir_b, mat);

        let diagonal1: AABB = AABB::from_point(orig, &(orig + dir_a + dir_b));
        let diagonal2: AABB = AABB::from_point(&(orig + dir_a), &(orig + dir_b));
        let bounding_box: AABB = AABB::from_bounding_box(&diagonal1, &diagonal2);
        plane.bounding_box = bounding_box;
        Self { plane }
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
}

impl Interior for Quad {
    fn is_interior(plane_coord: (f64, f64)) -> Option<(f64, f64)> {
        if !Interval::UNIT.contains(plane_coord.0) || !Interval::UNIT.contains(plane_coord.1) {
            return None;
        }
        Some(plane_coord)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut rec) = self.plane.hit(ray, ray_t) {
            if let Some(plane_coord) = Self::is_interior(self.plane.planar_hit_coordinates(&rec.point)) {
                // Ray hits the 2D shape; update hit record
                rec.uv = plane_coord;
                return Some(rec);
            }
            return None;
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.plane.bounding_box
    }

    fn pdf_value(&self, origin: &Point3f, direction: &Vec3f) -> f64 {
        self.plane.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3f) -> Vec3f {
        self.plane.random(origin)
    }
}


#[derive(Clone)]
pub struct Tri {
    plane: Plane,
}

impl fmt::Display for Tri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane: {}", self.plane)
    }
}

impl Tri {
    pub fn new(orig: &Point3f, dir_a: &Vec3f, dir_b: &Vec3f, mat: Arc<dyn Material>) -> Self {
        let mut plane: Plane = Plane::new(orig, dir_a, dir_b, mat);

        let diagonal1: AABB = AABB::from_point(orig, &(orig + dir_a + dir_b));
        let diagonal2: AABB = AABB::from_point(&(orig + dir_a), &(orig + dir_b));
        let bounding_box: AABB = AABB::from_bounding_box(&diagonal1, &diagonal2);
        plane.bounding_box = bounding_box;
        Self { plane }
    }
}

impl Interior for Tri {
    fn is_interior(plane_coord: (f64, f64)) -> Option<(f64, f64)> {
        if plane_coord.0 < 0.0 || plane_coord.1 < 0.0 || plane_coord.0 + plane_coord.1 > 1.0 {
            return None;
        }
        Some(plane_coord)
    }
}

impl Hittable for Tri {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut rec) = self.plane.hit(ray, ray_t) {
            if let Some(plane_coord) = Self::is_interior(self.plane.planar_hit_coordinates(&rec.point)) {
                // Ray hits the 2D shape; update hit record
                rec.uv = plane_coord;
                return Some(rec);
            }
            return None;
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.plane.bounding_box
    }

    fn pdf_value(&self, origin: &Point3f, direction: &Vec3f) -> f64 {
        self.plane.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3f) -> Vec3f {
        self.plane.random(origin)
    }
}