use std::fmt;
use std::sync::Arc;

use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bounding_box: AABB
}

impl Sphere {
    pub fn new_stationary(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec: Vec3 = radius * Vec3::ONE;
        Self {
            center: Ray::new(static_center, Vec3::ZERO),
            radius: radius.max(0.0),
            mat,
            bounding_box: AABB::from_point(&(static_center - rvec), &(static_center + rvec))
        }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center: Ray = Ray::new(center1, center2 - center1);
        let rvec: Vec3 = radius * Vec3::ONE;
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
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Center: {}; Radius: {}; Material: {}", self.center, self.radius, self.mat)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center: Point3 = self.center.at(ray.time());
        let oc: Vec3 = current_center - ray.origin();

        let a: f64 = ray.direction().length_squared();
        let h: f64 = Vec3::dot(&ray.direction(), &oc);
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

        let ray_root: Vec3 = ray.at(root);
        let outward_normal: Vec3 = (ray_root - current_center) / self.radius;
        let rec: HitRecord = HitRecord::new(ray_root, self.mat.clone(), root, ray, &outward_normal);
        
        Some(rec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}