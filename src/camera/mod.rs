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
    pub samples_per_pixel: u32,

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Camera {
        let image_height: u32 = max((image_width as f64 / aspect_ratio) as u32, 1);
        let pixel_samples_scale: f64 = 1.0 / (samples_per_pixel as f64);
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
            samples_per_pixel,
            image_height, 
            pixel_samples_scale,
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
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, world);
                }

                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let random_offset: Vec3 = Vec3::sample_unit_square();
        let pixel_sample = self.pixel00_loc
                            + (((i as f64) + random_offset.component(Axis::X)) * self.pixel_delta_u)
                            + (((j as f64) + random_offset.component(Axis::Y)) * self.pixel_delta_v);

        let ray_origin: Point3 = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        };
    
        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world);
        }
        
        let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
        let a: f64 = 0.5*(unit_direction.component(Axis::Y) + 1.0);
        
        (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}