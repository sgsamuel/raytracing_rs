use std::sync::Arc;

use super::aabb::AABB;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material::Lambertian;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bounding_box: AABB
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounding_box: AABB::EMPTY
        }
    }

    pub fn from_object(object: Arc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bounding_box = AABB::from_bounding_box(&self.bounding_box, object.bounding_box());
        self.objects.push(object);
    }
}


impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_rec: HitRecord = HitRecord {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            mat: Arc::new(Lambertian::new(Color::ZERO)),
            t: 0.0,
            front_face: false
        };
        
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in &self.objects {
            match object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                Some(rec) => {
                    hit_anything = true;
                    closest_so_far = rec.t;
                    hit_rec = rec;
                },
                None => ()   
            }
        }

        if hit_anything {
            return Some(hit_rec);
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}