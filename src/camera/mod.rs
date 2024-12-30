use std::cmp::max;
use log::info;

use super::color::{Color, write_color};
use super::hittable::{Hittable, HitRecord};
use super::interval::Interval;
use super::vec3::{Axis, Point3, Vec3};
use super::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Camera {
        let image_height: u32 = max((image_width as f64 / aspect_ratio) as u32, 1);
        let center: Point3 = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = center - Vec3::new(0.0, 0.0, focal_length) 
                                            - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Point3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera { 
            aspect_ratio,
            image_width,
            image_height, 
            center,
            pixel00_loc,
            pixel_delta_u, 
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");
    
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                info!("Scanlines remaining: {}", self.image_height - j);
                
                let pixel_center: Vec3 = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction: Vec3 = pixel_center - self.center;
                let camera_ray: Ray = Ray::new(self.center, ray_direction);
    
                let pixel_color: Color = self.ray_color(&camera_ray, world);
                write_color(pixel_color);
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        };
    
        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
        }
        
        let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
        let a: f64 = 0.5*(unit_direction.component(Axis::Y) + 1.0);
        
        (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}