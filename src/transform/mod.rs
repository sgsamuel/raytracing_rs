use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

struct Translation {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bounding_box: AABB
}

impl Translation {
    pub fn new(object: Arc<dyn Hittable>, offset: &Vec3) -> Self {
        let bounding_box: AABB = object.bounding_box() + offset;
        Self { object, offset: *offset, bounding_box }
    }
}

impl Hittable for Translation {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r: Ray = Ray::with_time(&(ray.origin() - self.offset), ray.direction(), ray.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            // Move the intersection point forwards by the offset
            rec.point += self.offset;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}