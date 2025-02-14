use std::cmp::Ordering;
use std::sync::Arc;
use std::fmt::{Display, Formatter};

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Axis;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB
}

impl Display for BVHNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BVHNode {{ bounding_box: {:?} }}", self.bounding_box))
    }
}

impl BVHNode {
    pub fn from_slice(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let mut bounding_box: AABB = AABB::EMPTY;
        for object in &mut *objects {
            bounding_box = AABB::from_bounding_box(&bounding_box, object.bounding_box());
        }

        let object_span: usize = objects.len();

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } 
        else if object_span == 2 {
            if Self::box_compare(&objects[0], &objects[1], bounding_box.longest_axis()) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } 
            else {
                left = objects[1].clone();
                right = objects[0].clone();  
            }
        } 
        else {
            let mid: usize = object_span / 2;
            let obj_slice = &mut objects[..];   
            obj_slice.select_nth_unstable_by(mid, 
                |a, b| {
                    Self::box_compare(a, b, bounding_box.longest_axis())
                }
            );

            left = Arc::new(BVHNode::from_slice(&mut objects[..mid]));
            right = Arc::new(BVHNode::from_slice(&mut objects[mid..]));
        }

        let bounding_box: AABB = AABB::from_bounding_box(left.bounding_box(), right.bounding_box());
        Self { left, right, bounding_box }
    }

    pub fn from_hittable_list(list: &mut HittableList) -> Self {
        Self::from_slice(&mut list.objects)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: Axis) -> Ordering {
        let a_axis_interval: Interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval: Interval = b.bounding_box().axis_interval(axis);
        
        if a_axis_interval.min < b_axis_interval.min {
            return Ordering::Less;
        }
        else if a_axis_interval.min > b_axis_interval.min {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, ray_t) {
            return None
        }

        let mut right_ray_max: f64 = ray_t.max;
        let hit_left: Option<HitRecord> = self.left.hit(ray, ray_t);
        if let Some(ref rec) = hit_left {
            right_ray_max = rec.t
        }
        let hit_right: Option<HitRecord>  = self.right.hit(ray, &Interval::new(ray_t.min, right_ray_max));
        
        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}