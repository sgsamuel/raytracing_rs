use std::cmp::Ordering;
use std::rc::Rc;

use super::aabb::AABB;
use super::hittable::Hittable;
use super::hittable_list::HittableList;
use super::interval::Interval;
use super::vec3::Axis;

#[derive(Clone)]
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AABB
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
        } 
        else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start+1].clone();
        } 
        else {
            let obj_slice = &mut objects[start..end];
            obj_slice.sort_by(
                |a, b| {
                    BVHNode::box_compare(a, b, bounding_box.longest_axis())
                }
            );

            let mid: usize = start + object_span/2;
            left = Rc::new(BVHNode::from_vector(objects, start, mid));
            right = Rc::new(BVHNode::from_vector(objects, mid, end));
        }

        let bounding_box = AABB::from_bounding_box(left.bounding_box(), right.bounding_box());

        Self { left, right, bounding_box }
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
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &mut Interval, rec: &mut crate::hittable::HitRecord) -> bool {
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

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}