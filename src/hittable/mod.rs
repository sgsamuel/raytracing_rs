use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, mat: Arc<dyn Material>, t: f64, uv: (f64, f64), ray: &Ray, outward_normal: &Vec3) -> Self {
        let front_face: bool = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal: Vec3 = if front_face {
            *outward_normal
        } 
        else {
            -*outward_normal
        };

        Self { point, normal, mat, t, uv, front_face }
    }
}

pub trait Hittable : Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}