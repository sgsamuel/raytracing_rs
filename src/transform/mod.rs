use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utilities;
use crate::vec3::{Axis, Point3f, Vec3f};

struct Translation {
    object: Arc<dyn Hittable>,
    offset: Vec3f,
    bounding_box: AABB
}

impl Translation {
    pub fn new(object: Arc<dyn Hittable>, offset: &Vec3f) -> Self {
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


struct Rotation {
    axis: Axis,
    cos_theta: f64,
    sin_theta: f64,
}

struct EulerRotation {
    object: Arc<dyn Hittable>,
    euler_angles: Vec3f<Rotation>,
    bounding_box: AABB
}

impl Rotation {
    pub fn new(object: Arc<dyn Hittable>, axis: Axis, degree_angle: f64) -> Self {
        let radians: f64 = utilities::degrees_to_radians(degree_angle);
        let cos_theta: f64 = f64::cos(radians);
        let sin_theta: f64 = f64::sin(radians);
        let mut bounding_box: &AABB = object.bounding_box();
        
        let mut min: Point3f = Point3f::INFINITY;
        let mut max: Point3f = -Point3f::INFINITY;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = (i as f64).mul_add(bounding_box.x.max, ((1 - i) as f64) * bounding_box.x.min);
                    let y: f64 = (j as f64).mul_add(bounding_box.y.max, ((1 - j) as f64) * bounding_box.y.min);
                    let z: f64 = (k as f64).mul_add(bounding_box.z.max, ((1 - k) as f64) * bounding_box.z.min);

                    let new_x: f64 =  cos_theta*x + sin_theta*z;
                    let new_z: f64 = -sin_theta*x + cos_theta*z;

                    let test_vec: Vec3f = Vec3f::new(new_x, y, new_z);

                    for &axis in Axis::iterator() {
                        min.x = f64::min(min.component(axis), test_vec.component(axis));
                        max[c] = std::fmax(max[c], tester[c]);
                    }
                }
            }
        }

//         let bounding_box: AABB = object.bounding_box() + offset;
//         Self { object, axis, cos_theta, sin_theta, bounding_box }
//     }
// }

// impl Hittable for Rotation {
//     fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
//         // Move the ray backwards by the offset
//         let offset_r: Ray = Ray::with_time(&(ray.origin() - self.offset), ray.direction(), ray.time());

//         // Determine whether an intersection exists along the offset ray (and if so, where)
//         if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
//             // Move the intersection point forwards by the offset
//             rec.point += self.offset;
//             return Some(rec);
//         }
//         None
//     }

//     fn bounding_box(&self) -> &AABB {
//         &self.bounding_box
//     }
// }