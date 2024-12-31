use std::path::Path;
use std::rc::Rc;

use dotenv::dotenv;
use env_logger;
use log::info;

pub mod color;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;


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

fn main() {
    dotenv().ok();
    env_logger::init();

    // Output
    let output_filepath = Path::new("test.ppm");

    // World
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0,0.0,-1.0), 0.5, material_right)));


    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 10;
    let cam: Camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    cam.render(&world, output_filepath);
    
    info!("Done");
}