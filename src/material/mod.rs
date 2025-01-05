use std::fmt;
use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{Texture, Solid};
use crate::utilities;
use crate::vec3::{Point3f, Vec3f};

pub trait Material: Send + Sync + fmt::Display {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _uv: (f64, f64), _point: &Point3f) -> Color {
        Color::ZERO
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
        let mut scatter_direction: Vec3f = rec.normal + Vec3f::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        let attenuation: Color = self.texture.value(rec.uv, &rec.point);
        let scattered: Ray = Ray::with_time(&rec.point, &scatter_direction, ray_in.time());
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
        let reflected: Vec3f = Vec3f::reflect(&Vec3f::unit_vector(ray_in.direction()), &rec.normal);
        let scattered_dir = reflected + self.fuzz * Vec3f::random_unit_vector();
        
        let scattered: Ray = Ray::with_time(&rec.point, &scattered_dir, ray_in.time());
        if Vec3f::dot(scattered.direction(), &rec.normal) > 0.0 {
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

        let unit_direction: Vec3f = Vec3f::unit_vector(ray_in.direction());
        let cos_theta: f64 = Vec3f::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3f = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utilities::random() {
            Vec3f::reflect(&unit_direction, &rec.normal)
        } 
        else {            
            Vec3f::refract(&unit_direction, &rec.normal, ri)
        };

        let attenuation: Color = Color::ONE;
        let scattered: Ray = Ray::with_time(&rec.point, &direction, ray_in.time());
        Some((attenuation, scattered))
    }
}


pub struct DiffuseLight {
    texture: Arc<dyn Texture>
}

impl fmt::Display for DiffuseLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material DiffuseLight. Texture: {}", self.texture)
    }
}

impl DiffuseLight {
    pub fn from_color(emit: &Color) -> Self {
        Self { texture: Arc::new(Solid::new(emit)) }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: (f64, f64), point: &Point3f) -> Color {
        self.texture.value(uv, point)
    }
}


pub struct Isotropic {
    texture: Arc<dyn Texture>
}

impl fmt::Display for Isotropic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material Isotropic. Texture: {}", self.texture)
    }
}

impl Isotropic {
    pub fn from_color(albedo: &Color) -> Self {
        Self { texture: Arc::new(Solid::new(albedo)) }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation: Color = self.texture.value(rec.uv, &rec.point);
        let scattered: Ray = Ray::with_time(&rec.point, &Vec3f::random_unit_vector(), ray_in.time());
        Some((attenuation, scattered))
    }
}