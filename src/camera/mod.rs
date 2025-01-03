use std::cmp::max;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

use log::info;
use rayon::prelude::*;

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::utilities;
use crate::vec3::{Axis, Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,          // Ratio of image width over height
    pub image_width: u32,           // Rendered image width in pixel count
    pub samples_per_pixel: u32,     // Count of random samples for each pixel
    pub max_depth: u32,             // Maximum number of ray bounces into scene

    pub vertical_fov: f64,          // Vertical view angle (field of view)
    pub lookfrom: Point3,           // Point camera is looking from
    pub lookat: Point3,             // Point camera is looking at
    pub vup: Vec3,                  // Camera-relative "up" direction
    pub defocus_angle: f64,         // Variation angle of rays through each pixel
    pub focus_dist: f64,            // Distance from camera lookfrom point to plane of perfect focus

    image_height: u32,              // Rendered image height
    pixel_samples_scale: f64,       // Color scale factor for a sum of pixel samples
    center: Point3,                 // Camera center
    pixel00_loc: Point3,            // Location of pixel (0, 0)
    pixel_delta_u: Vec3,            // Offset to pixel to the right
    pixel_delta_v: Vec3,            // Offset to pixel below
    defocus_disk_u: Vec3,           // Defocus disk horizontal radius
    defocus_disk_v: Vec3            // Defocus disk vertical radius
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_ratio: f64, 
        image_width: u32, 
        samples_per_pixel: u32, 
        max_depth: u32, 
        vertical_fov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64
    ) -> Self {
        let image_height: u32 = max((image_width as f64 / aspect_ratio) as u32, 1);
        let pixel_samples_scale: f64 = 1.0 / (samples_per_pixel as f64);
        let center: Point3 = lookfrom;

        // Determine viewport dimensions.
        let theta: f64 = utilities::degrees_to_radians(vertical_fov);
        let h: f64 = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w: Vec3 = Vec3::unit_vector(&(lookfrom - lookat));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = viewport_width * u;      // Vector across viewport horizontal edge
        let viewport_v: Vec3 = viewport_height * -v;    // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Point3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius: f64 = focus_dist * f64::tan(utilities::degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u: Vec3 = u * defocus_radius;
        let defocus_disk_v: Vec3 = v * defocus_radius;

        Self { 
            aspect_ratio, image_width, samples_per_pixel, max_depth,
            vertical_fov, lookfrom, lookat, vup,
            defocus_angle, focus_dist,
            image_height, pixel_samples_scale, center, 
            pixel00_loc, pixel_delta_u, pixel_delta_v,
            defocus_disk_u, defocus_disk_v
        }
    }

    pub fn render(&self, world: &HittableList, output_filepath: &Path) {
        let file: File = File::create(output_filepath).unwrap(); 
        let mut writer: BufWriter<File> = BufWriter::new(file);

        info!("Generating image");
        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{} {}", self.image_width, self.image_height).unwrap();
        writeln!(writer, "255").unwrap();
    
        let pixels = (0..self.image_height).into_par_iter().map(
            |j: u32| {
                info!("Scanline: {}", j);
                (0..self.image_width).into_par_iter().map(
                    |i: u32| {
                        let mut pixel_color: Color = Color::ZERO;
                        for _ in 0..self.samples_per_pixel {
                            let ray: Ray = self.get_ray(i, j);
                            pixel_color += Self::ray_color(&ray, self.max_depth, world);
                        }
        
                        write_color(self.pixel_samples_scale * pixel_color)
                    }
                ).collect::<Vec<String>>().join("")
            }
        ).collect::<Vec<String>>().join("");

        writeln!(writer, "{}", pixels).unwrap();
        writer.flush().unwrap();
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let random_offset: Vec3 = Vec3::sample_unit_square();
        let pixel_sample: Vec3 = self.pixel00_loc
                            + (((i as f64) + random_offset.component(Axis::X)) * self.pixel_delta_u)
                            + (((j as f64) + random_offset.component(Axis::Y)) * self.pixel_delta_v);

        let ray_origin: Point3 = if self.defocus_angle <= 0.0 {
            self.center
        }
        else {
            self.defocus_disk_sample()
        };

        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time = utilities::random();

        Ray::with_time(&ray_origin, &ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p: Vec3 = Vec3::random_in_unit_disk();
        self.center + (p.component(Axis::X) * self.defocus_disk_u) + (p.component(Axis::Y) * self.defocus_disk_v)
    }

    fn ray_color(ray: &Ray, depth: u32, world: &HittableList) -> Color {        
        if depth == 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            match rec.mat.scatter(ray, &rec) { 
                Some((attenuation, scattered)) => {
                    return attenuation * Self::ray_color(&scattered, depth-1, world)
                },
                None => {
                    return Color::ZERO;
                }
            }
        }
        
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
        let a: f64 = 0.5*(unit_direction.component(Axis::Y) + 1.0);
        
        (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
    }
}