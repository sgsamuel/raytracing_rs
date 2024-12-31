use std::fmt;

use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::utilities;
use super::vec3::Vec3;

pub trait Material: fmt::Display {
    fn scatter(
        &self, _ray_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray
    ) -> bool {
        false
    }
}


pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl fmt::Display for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Lambertian. Albedo: {}", self.albedo)
    }
}


pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal);
        let scattered_dir = reflected + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scattered_dir);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &rec.normal) > 0.0
    }
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Metal. Albedo: {}; Fuzz: {}", self.albedo, self.fuzz)
    }
}


pub struct Dielectric {
    refractive_index: f64
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
    fn scatter(
        &self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray
    ) -> bool {
        let ri: f64;
        if rec.front_face {
            ri = 1.0 / self.refractive_index;
        } 
        else {
            ri = self.refractive_index;
        }

        let unit_direction: Vec3 = Vec3::unit_vector(ray_in.direction());
        let cos_theta: f64 = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utilities::random() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } 
        else {            
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }

        *attenuation = Color::ONE;
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

impl fmt::Display for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Dielectric. Refractive Index: {}", self.refractive_index)
    }
}