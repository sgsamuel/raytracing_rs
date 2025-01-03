use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use dotenv::dotenv;
use env_logger;
use log::info;

pub mod aabb;
pub mod bvh_node;
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

use bvh_node::BVHNode;
use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use vec3::{Point3, Vec3};


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

#[allow(dead_code)]
fn simple_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let material_ground: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<Dielectric>   = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<Dielectric> = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Arc<Metal>       = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.2));

    scene.add(Arc::new(Sphere::new_stationary(Point3::new(0.0,-100.5,-1.0), 100.0, material_ground)));
    scene.add(Arc::new(Sphere::new_stationary(Point3::new(0.0,0.0,-1.2), 0.5, material_center)));
    scene.add(Arc::new(Sphere::new_stationary(Point3::new(-1.0,0.0,-1.0), 0.5, material_left)));
    scene.add(Arc::new(Sphere::new_stationary(Point3::new(-1.0,0.0,-1.0), 0.4, material_bubble)));
    scene.add(Arc::new(Sphere::new_stationary(Point3::new(1.0,0.0,-1.0), 0.5, material_right)));


    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3        = Point3::new(-2.0, 2.0, 1.0);
    let lookat: Point3          = Point3::new(0.0, 0.0, -1.0);
    let vup: Vec3               = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 10.0;
    let focus_dist: f64         = 3.4;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, max_depth, 
        vertical_fov, lookfrom, lookat, vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
fn bouncing_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let ground_material : Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add(Arc::new(Sphere::new_stationary(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = utilities::random();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * utilities::random(), 
                0.2, 
                b as f64 + 0.9 * utilities::random()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // Lambertian
                    let albedo: Vec3 = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2: Point3 = center + Vec3::new(0.0, utilities::random_f64_range(0.0, 0.5), 0.0);
                    scene.add(Arc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                } 
                else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = utilities::random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    scene.add(Arc::new(Sphere::new_stationary(center, 0.2, sphere_material)));
                } 
                else {
                    // Dielectric
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    scene.add(Arc::new(Sphere::new_stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let dielectric_material: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    scene.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0), 1.0, dielectric_material)));

    let lambertian_material: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0), 1.0, lambertian_material)));

    let metal_material: Arc<Metal> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0), 1.0, metal_material)));


    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3        = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3          = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3               = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.6;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, max_depth, 
        vertical_fov, lookfrom, lookat, vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let now: Instant = Instant::now();

    // Output
    let output_filepath: &Path = Path::new("test.ppm");

    // World + Camera
    let (mut scene, cam) = bouncing_spheres();
    let bvh_scene: Arc<BVHNode> = Arc::new(BVHNode::from_hittable_list(&mut scene));
    let world: HittableList = HittableList::from_object(bvh_scene);

    cam.render(&world, output_filepath);
    
    let elapsed: Duration = now.elapsed();
    info!("Done. Time elapsed {:.2?}", elapsed);
}