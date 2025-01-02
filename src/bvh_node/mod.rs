use std::cmp::Ordering;
use std::rc::Rc;
use std::fmt::{Display, Formatter};

use log::debug;

use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::hittable_list::HittableList;
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::Axis;

#[derive(Clone)]
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AABB
}

impl Display for BVHNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BVH {{ bbox: {:?} }}", self.bounding_box))
    }
}

impl BVHNode {
    pub fn from_vector(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bounding_box: AABB = AABB::EMPTY;
        for object in &mut *objects {
            bounding_box = AABB::from_bounding_box(&bounding_box, object.bounding_box());
        }

        let object_span: usize = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
            debug!("1 Start {}; End {}", start, end);
        } 
        else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
            debug!("2 Start {}; End {}", start, end);
        } 
        else {
            let obj_slice = &mut objects[start..end];   
            obj_slice.sort_by(
                |a, b| {
                    BVHNode::box_compare(a, b, bounding_box.longest_axis())
                }
            );

            let mid: usize = start + object_span / 2;
            left = Rc::new(BVHNode::from_vector(objects, start, mid));
            right = Rc::new(BVHNode::from_vector(objects, mid, end));
            
            debug!("Split Start {}; End {}", start, end);
        }

        let bounding_box: AABB = AABB::from_bounding_box(left.bounding_box(), right.bounding_box());

        let obj: BVHNode = Self { left, right, bounding_box };
        debug!("BVH Node Bounding Box: {}", obj);
        obj
    }

    pub fn from_hittable_list(list: &mut HittableList) -> Self {
        let hittable_list_len: usize = list.objects.len();
        Self::from_vector(&mut list.objects, 0, hittable_list_len)
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: Axis) -> Ordering {
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
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, ray_t) {
            return false
        }

        let hit_left: bool = self.left.hit(ray, ray_t, rec);
        let right_ray_max: f64;
        if hit_left {
            right_ray_max = rec.t
        }
        else {
            right_ray_max = ray_t.max;
        }
        let hit_right: bool = self.right.hit(ray, &mut Interval::new(ray_t.min, right_ray_max), rec);
        
        hit_right || hit_left
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}