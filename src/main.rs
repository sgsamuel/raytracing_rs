use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use dotenv::dotenv;
use log::info;

pub mod aabb;
pub mod bvh_node;
pub mod color;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod utilities;
pub mod vec3;

use bvh_node::BVHNode;
use hittable_list::HittableList;


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

    let now: Instant = Instant::now();

    // Output
    let output_filepath: &Path = Path::new("test.ppm");

    // World + Camera
    let (mut scene, cam) = scenes::cornell_box();
    let bvh_scene: Arc<BVHNode> = Arc::new(BVHNode::from_hittable_list(&mut scene));
    let world: HittableList = HittableList::from_object(bvh_scene);

    cam.render(&world, output_filepath);
    
    let elapsed: Duration = now.elapsed();
    info!("Done. Time elapsed {:.2?}", elapsed);
}