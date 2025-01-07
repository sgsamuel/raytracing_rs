use core::f64;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Material, Isotropic};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utilities;
use crate::vec3::Vec3f;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, color: &Color) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::from_color(color)) }
    }

    pub fn from_texture(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::from_texture(texture)) }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, &Interval::UNIVERSE) {
            if let Some(mut rec2) = self.boundary.hit(ray, &Interval::new(rec1.t + 0.0001, f64::INFINITY)) {
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
        
                if rec1.t >= rec2.t {
                    return None;
                }
        
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = ray.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * f64::ln(utilities::random());

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t: f64 = rec1.t + hit_distance / ray_length;
                let rec: HitRecord = HitRecord { 
                    point: ray.at(t), 
                    normal: Vec3f::E1, // Arbitrary 
                    mat: self.phase_function.clone(), 
                    t,
                    uv: (0.0, 0.0), // Arbitrary
                    front_face: true // Arbitrary
                };

                return Some(rec);  
            }
            return None;
        }
        None   
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
