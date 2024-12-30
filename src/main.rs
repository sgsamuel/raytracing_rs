use std::rc::Rc;

use dotenv::dotenv;
use env_logger;
use log::info;

pub mod color;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

use camera::Camera;
use hittable_list::HittableList;
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

    // World
    let mut world: HittableList = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));


    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let cam: Camera = Camera::new(aspect_ratio, image_width);

    cam.render(&world);
    
    info!("Done");
}