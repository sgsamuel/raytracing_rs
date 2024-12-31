use std::rc::Rc;

use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        if Vec3::dot(ray.direction(), outward_normal) < 0.0 {
            self.normal = *outward_normal;
        } 
        else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}