use std::rc::Rc;

use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material::Lambertian;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from_object(object: Rc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}


impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            mat: Rc::new(Lambertian::new(Color::ZERO)),
            t: 0.0,
            front_face: false
        };

        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}