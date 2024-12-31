use std::fmt;

use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
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
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
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
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.max(1.0)
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