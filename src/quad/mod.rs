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
    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)>;
}

#[derive(Clone)]
pub struct PlanarHitRecord {
    pub intersection_pt: Point3f,
    pub t: f64,
    pub alpha: f64,
    pub beta: f64
}

#[derive(Clone)]
pub struct Plane {
    corner: Point3f,
    side_a: Vec3f,
    side_b: Vec3f,
    w: Vec3f,
    area: f64,
    quad_eq_d: f64,
    normal: Vec3f,
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Corner: {}; Side A: {}; Side B: {}", self.corner, self.side_a, self.side_b)
    }
}

impl Plane {
    pub fn new(corner: &Point3f, side_a: &Vec3f, side_b: &Vec3f) -> Self {
        let n: Vec3f = Vec3f::cross(side_a, side_b);
        let normal: &Vec3f = &Vec3f::unit_vector(&n);
        let w: Vec3f = n / Vec3f::length_squared(&n);
        let area: f64 = n.length();
        let quad_eq_d: f64 = Vec3f::dot(normal, corner);

        Self { 
            corner: *corner, 
            side_a: *side_a,
            side_b: *side_b,
            w,
            area,
            quad_eq_d, 
            normal: *normal
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<PlanarHitRecord>  {
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
        let intersection_pt: Vec3f = ray.at(t);
        let planar_hitpt_vector: Vec3f = intersection_pt - self.corner;
        let alpha: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&planar_hitpt_vector, &self.side_b));
        let beta: f64 = Vec3f::dot(&self.w, &Vec3f::cross(&self.side_a, &planar_hitpt_vector));

        let planar_rec = PlanarHitRecord { intersection_pt, t, alpha, beta };
        Some(planar_rec)
    }
}

#[derive(Clone)]
pub struct Quad {
    plane: Plane,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane: {}; Material: {}", self, self.mat)
    }
}

impl Quad {
    pub fn new(corner: &Point3f, side_a: &Vec3f, side_b: &Vec3f, mat: Arc<dyn Material>) -> Self {
        let plane: Plane = Plane::new(corner, side_a, side_b);

        let diagonal1: AABB = AABB::from_point(corner, &(corner + side_a + side_b));
        let diagonal2: AABB = AABB::from_point(&(corner + side_a), &(corner + side_b));
        let bounding_box = AABB::from_bounding_box(&diagonal1, &diagonal2);
        Self { 
            plane,
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
}

impl Interior for Quad {
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
        if let Some(planar_rec) = self.plane.hit(ray, ray_t) {
            if let Some(uv) = Self::is_interior(planar_rec.alpha, planar_rec.beta) {
                // Ray hits the 2D shape; set the hit record
                let rec: HitRecord = HitRecord::new(
                    planar_rec.intersection_pt, 
                    self.mat.clone(), 
                    planar_rec.t, 
                    uv, 
                    ray, 
                    &self.plane.normal
                );  
                return Some(rec);
            }
            return None;
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    fn pdf_value(&self, origin: &Point3f, direction: &Vec3f) -> f64 {
        if let Some(rec) =  self.hit(&Ray::new(origin, direction), &Interval::new(0.001, f64::INFINITY)) {
            let distance_squared: f64 = rec.t * rec.t * direction.length_squared();
            let cos_theta: f64 = f64::abs(Vec3f::dot(direction, &rec.normal) / direction.length());
            return distance_squared / (cos_theta * self.plane.area);
        }
        0.0
    }

    fn random(&self, origin: &Point3f) -> Vec3f {
        let p: Vec3f = self.plane.corner + (random() * self.plane.side_a) + (random() * self.plane.side_b);
        p - *origin
    }
}