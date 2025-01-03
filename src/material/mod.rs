use std::fmt;
use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{Texture, Solid};
use crate::utilities;
use crate::vec3::Vec3;

pub trait Material: Send + Sync + fmt::Display {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}


pub struct Lambertian {
    texture: Arc<dyn Texture>
}

impl fmt::Display for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Lambertian. Texture: {}", self.texture)
    }
}

impl Lambertian {
    pub fn from_color(albedo: &Color) -> Self {
        Self { texture: Arc::new(Solid::new(albedo)) }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        let attenuation: Color = self.texture.value(rec.uv, &rec.p);
        let scattered: Ray = Ray::with_time(&rec.p, &scatter_direction, ray_in.time());
        Some((attenuation, scattered))
    }
}


pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Metal. Albedo: {}; Fuzz: {}", self.albedo, self.fuzz)
    }
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal);
        let scattered_dir = reflected + self.fuzz * Vec3::random_unit_vector();
        
        let scattered: Ray = Ray::with_time(&rec.p, &scattered_dir, ray_in.time());
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            let attenuation: Color = self.albedo;
            return Some((attenuation, scattered))
        }  
        None
    }
}


pub struct Dielectric {
    refractive_index: f64
}

impl fmt::Display for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Dielectric. Refractive Index: {}", self.refractive_index)
    }
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0: f64 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5) 
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let ri: f64 = if rec.front_face {
            1.0 / self.refractive_index
        } 
        else {
            self.refractive_index
        };

        let unit_direction: Vec3 = Vec3::unit_vector(ray_in.direction());
        let cos_theta: f64 = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utilities::random() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } 
        else {            
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        let attenuation: Color = Color::ONE;
        let scattered: Ray = Ray::with_time(&rec.p, &direction, ray_in.time());
        Some((attenuation, scattered))
    }
}