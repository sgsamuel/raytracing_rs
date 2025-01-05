use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utilities;
use crate::vec3::{Axis, Point3f, Vec3f};

pub struct Translation {
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


#[derive(Clone, Copy)]
pub struct AxisRotation;

impl AxisRotation {
    fn rotate(axis: Axis, point: &Point3f, radian: f64) -> Point3f {
        match axis {
            Axis::X => {
                Point3f::new(
                    point.component(Axis::X),
                    radian.cos().mul_add(point.component(Axis::Y), -radian.sin() * point.component(Axis::Z)),
                    radian.sin().mul_add(point.component(Axis::Y), radian.cos() * point.component(Axis::Z))
                )
            },
            Axis::Y => {
                Point3f::new(
                    radian.cos().mul_add(point.component(Axis::X), radian.sin() * point.component(Axis::Z)),
                    point.component(Axis::Y),
                    (-radian.sin()).mul_add(point.component(Axis::X), radian.cos() * point.component(Axis::Z))
                )
            },
            Axis::Z => {
                Point3f::new(
                    radian.cos().mul_add(point.component(Axis::X), -radian.sin() * point.component(Axis::Y)),
                    radian.sin().mul_add(point.component(Axis::X), radian.cos() * point.component(Axis::Y)),
                    point.component(Axis::Z)
                )
            }
        }
    }
}

pub struct EulerRotation {
    object: Arc<dyn Hittable>,
    euler_angles: Vec3f,
    bounding_box: AABB
}

impl EulerRotation {
    pub fn new(object: Arc<dyn Hittable>, angles: &Vec3f) -> Self {
        let mut euler_angles: Vec3f = Default::default();
        for &axis in Axis::iterator() {
            euler_angles.set_component(axis, utilities::degrees_to_radians(angles.component(axis)));
        }

        let bounding_box: &AABB = object.bounding_box();
        let mut point_min: Point3f = Point3f::INFINITY;
        let mut point_max: Point3f = -Point3f::INFINITY;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = (i as f64).mul_add(bounding_box.x.max, ((1 - i) as f64) * bounding_box.x.min);
                    let y: f64 = (j as f64).mul_add(bounding_box.y.max, ((1 - j) as f64) * bounding_box.y.min);
                    let z: f64 = (k as f64).mul_add(bounding_box.z.max, ((1 - k) as f64) * bounding_box.z.min);

                    let mut rotated_point: Point3f = Point3f::new(x, y, z);
                    for &axis in Axis::iterator() {
                        rotated_point = AxisRotation::rotate(axis, &rotated_point, euler_angles.component(axis));
                    }

                    for &axis in Axis::iterator() {
                        point_min.set_component(axis, f64::min(point_min.component(axis), rotated_point.component(axis)));
                        point_max.set_component(axis, f64::max(point_max.component(axis), rotated_point.component(axis)));
                    }
                }
            }
        }

        Self { object, euler_angles, bounding_box: AABB::from_point(&point_min, &point_max) }
    }
}

impl Hittable for EulerRotation {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let mut rotated_origin: Point3f = ray.origin().clone();
        let mut rotated_direction: Vec3f = ray.direction().clone();
        for &axis in Axis::iterator() {
            rotated_origin = AxisRotation::rotate(axis, &rotated_origin, -self.euler_angles.component(axis));
            rotated_direction = AxisRotation::rotate(axis, &rotated_direction, -self.euler_angles.component(axis));
        }

        let rotated_ray: Ray = Ray::new(&rotated_origin, &rotated_direction);

        // Determine whether an intersection exists in object space (and if so, where).
        if let Some(mut rec) = self.object.hit(&rotated_ray, ray_t) {
            // Transform the intersection from object space back to world space.

            for &axis in Axis::iterator() {
                rec.point = AxisRotation::rotate(axis, &rec.point , self.euler_angles.component(axis));
                rec.normal = AxisRotation::rotate(axis, &rec.normal, self.euler_angles.component(axis));
            }

            return Some(rec);
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}