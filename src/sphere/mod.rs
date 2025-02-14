use core::f64;
use std::fmt;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::utilities;
use crate::vec3::{Axis, Point3f, Vec3f};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Center: {}; Radius: {}; Material: {}", self.center, self.radius, self.mat)
    }
}

impl Sphere {
    pub fn new_stationary(static_center: &Point3f, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec: Vec3f = radius * Vec3f::ONE;
        Self {
            center: Ray::new(static_center, &Vec3f::ZERO),
            radius: radius.max(0.0),
            mat,
            bounding_box: AABB::from_point(&(static_center - rvec), &(static_center + rvec))
        }
    }

    pub fn new_moving(center1: &Point3f, center2: &Point3f, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center: Ray = Ray::new(center1, &(center2 - center1));
        let rvec: Vec3f = radius * Vec3f::ONE;
        let box1: AABB = AABB::from_point(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let box2: AABB = AABB::from_point(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));
        let bounding_box: AABB = AABB::from_bounding_box(&box1, &box2);
        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bounding_box
        }
    }

    fn get_sphere_uv(p: &Point3f) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     (1, 0, 0) yields (0.50, 0.50)       (-1,  0,  0) yields (0.00, 0.50)
        //     (0, 1, 0) yields (0.50, 1.00)       ( 0, -1,  0) yields (0.50, 0.00)
        //     (0, 0, 1) yields (0.25, 0.50)       ( 0,  0, -1) yields (0.75, 0.50)

        let theta: f64 = f64::acos(-p.component(Axis::Y));
        let phi: f64 = f64::atan2(-p.component(Axis::Z), p.component(Axis::X)) + f64::consts::PI;

        (phi / (2.0 * f64::consts::PI), theta / f64::consts::PI)
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3f {
        let r1: f64 = utilities::random();
        let r2: f64 = utilities::random();
        let z: f64 = 1.0 + r2 * (f64::sqrt(1.0 - radius*radius / distance_squared) - 1.0);

        let phi: f64 = 2.0 * f64::consts::PI * r1;
        let x: f64 = f64::cos(phi) * f64::sqrt(1.0 - z*z);
        let y: f64 = f64::sin(phi) * f64::sqrt(1.0 - z*z);

        Vec3f::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center: Point3f = self.center.at(ray.time());
        let oc: Vec3f = current_center - ray.origin();

        let a: f64 = ray.direction().length_squared();
        let h: f64 = Vec3f::dot(ray.direction(), &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let ray_root: Vec3f = ray.at(root);
        let outward_normal: Vec3f = (ray_root - current_center) / self.radius;
        let uv: (f64, f64) = Self::get_sphere_uv(&outward_normal);
        let rec: HitRecord = HitRecord::new(
            ray_root, 
            self.mat.clone(), 
            root, 
            uv, 
            ray, 
            &outward_normal
        );
        
        Some(rec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    fn pdf_value(&self, origin: &Point3f, direction: &Vec3f) -> f64 {
        // Only works for stationary spheres

        if let Some(_rec) = self.hit(&Ray::new(origin, direction), &Interval::new(0.001, f64::INFINITY)) {
            let dist_squared: f64 = (self.center.at(0.0) - origin).length_squared();
            let cos_theta_max: f64 = f64::sqrt(1.0 - self.radius * self.radius / dist_squared);
            let solid_angle: f64 = 2.0 * f64::consts::PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        }
        0.0
    }

    fn random(&self, origin: &Point3f) -> Vec3f {
        let direction: Vec3f = self.center.at(0.0) - origin;
        let distance_squared: f64 = direction.length_squared();
        let uvw: ONB = ONB::new(&direction);
        uvw.transform(&Self::random_to_sphere(self.radius, distance_squared))
    }
}