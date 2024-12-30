use std::cmp::max;
use dotenv::dotenv;
use env_logger;
use log::info;

pub mod color;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

use color::Color;
use ray::Ray;
use sphere::Sphere;
use vec3::{Axis, Point3, Vec3};


#[derive(Debug)]
pub struct Config {
    pub rust_log: String,
}

impl Config {
    pub fn init() -> Config {
        Config {
            rust_log: std::env::var("RUST_LOG")
                .expect("RUST_LOG must be specified"),
        }
    }
}


pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = center - ray.origin();

    let a: f64 = ray.direction().length_squared();
    let h: f64 = Vec3::dot(&ray.direction(), &oc);
    let c: f64 = oc.length_squared() - radius*radius;

    let discriminant: f64 = h*h - a*c;
    if discriminant < 0.0 {
        return -1.0;
    }

    (h - discriminant.sqrt()) / a
}

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal: Vec3 = Vec3::unit_vector(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5*Color::new(normal.component(Axis::X) + 1.0, normal.component(Axis::Y) + 1.0, normal.component(Axis::Z) + 1.0);
    }
    
    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let a: f64 = 0.5*(unit_direction.component(Axis::Y) + 1.0);
    
    (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}

fn main() {
    dotenv().ok();
    env_logger::init();


    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height: u32 = max((image_width as f64 / aspect_ratio) as u32, 1);
    
    
    // Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Vec3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 = camera_center
                                - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            info!("Scanlines remaining: {}", image_height - j);
            
            let pixel_center: Vec3 = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;
            let camera_ray: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&camera_ray);
            color::write_color(pixel_color);
        }
    }
    
    info!("Done");
}