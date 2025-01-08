use std::cmp::max;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use log::info;
use rayon::prelude::*;

use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::utilities;
use crate::vec3::{Axis, Point3f, Vec3f};
use crate::pdf::{HittablePDF, MixturePDF, PDF};
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,          // Ratio of image width over height
    pub image_width: u32,           // Rendered image width in pixel count
    pub samples_per_pixel: u32,     // Count of random samples for each pixel
    pub max_depth: u32,             // Maximum number of ray bounces into scene
    pub background: Color,          // Scene background color

    pub vertical_fov: f64,          // Vertical view angle (field of view)
    pub lookfrom: Point3f,          // Point camera is looking from
    pub lookat: Point3f,            // Point camera is looking at
    pub vup: Vec3f,                 // Camera-relative "up" direction
    pub defocus_angle: f64,         // Variation angle of rays through each pixel
    pub focus_dist: f64,            // Distance from camera lookfrom point to plane of perfect focus

    image_height: u32,              // Rendered image height
    pixel_samples_scale: f64,       // Color scale factor for a sum of pixel samples
    sqrt_spp: u32,                  // Square root of number of samples per pixel
    recip_sqrt_spp: f64,            // 1 / sqrt_spp
    center: Point3f,                // Camera center
    pixel00_loc: Point3f,           // Location of pixel (0, 0)
    pixel_delta_u: Vec3f,           // Offset to pixel to the right
    pixel_delta_v: Vec3f,           // Offset to pixel below
    defocus_disk_u: Vec3f,          // Defocus disk horizontal radius
    defocus_disk_v: Vec3f           // Defocus disk vertical radius
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_ratio: f64, 
        image_width: u32, 
        samples_per_pixel: u32, 
        max_depth: u32,
        background: &Color,
        vertical_fov: f64,
        lookfrom: &Point3f,
        lookat: &Point3f,
        vup: &Vec3f,
        defocus_angle: f64,
        focus_dist: f64
    ) -> Self {
        let image_height: u32 = max((image_width as f64 / aspect_ratio) as u32, 1);
        let sqrt_spp: u32 = f64::sqrt(samples_per_pixel as f64) as u32;
        let recip_sqrt_spp: f64 = 1.0 / (sqrt_spp as f64);
        let pixel_samples_scale: f64 = 1.0 / ((sqrt_spp * sqrt_spp) as f64);
        let center: Point3f = *lookfrom;

        // Determine viewport dimensions.
        let theta: f64 = utilities::degrees_to_radians(vertical_fov);
        let h: f64 = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w: Vec3f = Vec3f::unit_vector(&(lookfrom - lookat));
        let u: Vec3f = Vec3f::unit_vector(&Vec3f::cross(vup, &w));
        let v: Vec3f = Vec3f::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3f = viewport_width * u;      // Vector across viewport horizontal edge
        let viewport_v: Vec3f = viewport_height * -v;    // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: Vec3f = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3f = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3f = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Point3f = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius: f64 = focus_dist * f64::tan(utilities::degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u: Vec3f = u * defocus_radius;
        let defocus_disk_v: Vec3f = v * defocus_radius;

        Self { 
            aspect_ratio, image_width, samples_per_pixel, max_depth, 
            background: *background, vertical_fov, 
            lookfrom: *lookfrom, lookat: *lookat, vup: *vup,
            defocus_angle, focus_dist,
            image_height, pixel_samples_scale, sqrt_spp, recip_sqrt_spp, 
            center, pixel00_loc, pixel_delta_u, pixel_delta_v,
            defocus_disk_u, defocus_disk_v
        }
    }

    pub fn render(&self, world: &HittableList, lights: &HittableList, output_filepath: &Path) {
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
                        pixel_color += (0..self.sqrt_spp).into_par_iter().map(
                            |s_j: u32| {
                                (0..self.sqrt_spp).into_par_iter().map(
                                    |s_i: u32| {
                                        let r: Ray = self.get_ray(i, j, s_i, s_j);
                                        self.ray_color(&r, self.max_depth, world, lights)
                                    }
                                ).sum::<Color>()
                            }
                        ).sum::<Color>();
        
                        write_color(self.pixel_samples_scale * pixel_color)
                    }
                ).collect::<Vec<String>>().join("")
            }
        ).collect::<Vec<String>>().join("");

        writeln!(writer, "{}", pixels).unwrap();
        writer.flush().unwrap();
    }

    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j for stratified sample square s_i, s_j.
        let offset: Vec3f = self.sample_square_stratified(s_i, s_j);
        let pixel_sample: Vec3f = self.pixel00_loc
                            + (((i as f64) + offset.component(Axis::X)) * self.pixel_delta_u)
                            + (((j as f64) + offset.component(Axis::Y)) * self.pixel_delta_v);

        let ray_origin: Point3f = if self.defocus_angle <= 0.0 {
            self.center
        }
        else {
            self.defocus_disk_sample()
        };

        let ray_direction: Vec3f = pixel_sample - ray_origin;
        let ray_time = utilities::random();

        Ray::with_time(&ray_origin, &ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3f {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].
        let px: f64 = ((s_i as f64 + utilities::random()) * self.recip_sqrt_spp) - 0.5;
        let py: f64 = ((s_j as f64 + utilities::random()) * self.recip_sqrt_spp) - 0.5;

        Vec3f::new(px, py, 0.0)  
    }

    fn defocus_disk_sample(&self) -> Point3f {
        // Returns a random point in the camera defocus disk.
        let p: Vec3f = Vec3f::random_in_unit_disk();
        self.center + (p.component(Axis::X) * self.defocus_disk_u) + (p.component(Axis::Y) * self.defocus_disk_v)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList, lights: &HittableList) -> Color {        
        if depth == 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            let color_from_emission: Color = rec.mat.emitted(ray, &rec, rec.uv, &rec.point);
            if let Some(scatter_rec) = rec.mat.scatter(ray, &rec) {
                if scatter_rec.skip_pdf {
                    return scatter_rec.attenuation * self.ray_color(&scatter_rec.skip_pdf_ray, depth-1, world, lights);
                }

                let selected_pdf: Arc<dyn PDF>;
                if lights.objects.len() > 0 {
                    let light_pdf_ptr: Arc<HittablePDF>  = Arc::new(HittablePDF::new(Arc::new(lights.clone()), &rec.point));
                    selected_pdf = Arc::new(MixturePDF::new(light_pdf_ptr, scatter_rec.pdf_ptr));
                }
                else {
                    selected_pdf = scatter_rec.pdf_ptr;
                }


                let scattered: Ray = Ray::with_time(&rec.point, &selected_pdf.generate(), ray.time());
                let pdf_value: f64 = selected_pdf.value(scattered.direction());

                let scattering_pdf: f64 = rec.mat.scattering_pdf(ray, &rec, &scattered);

                let sample_color: Color = self.ray_color(&scattered, depth-1, world, lights);
                let color_from_scatter: Color = (scatter_rec.attenuation * scattering_pdf * sample_color) / pdf_value;
                return color_from_emission + color_from_scatter;
            }
            return color_from_emission;
        }

        self.background
    }
}