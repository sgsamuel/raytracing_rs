use core::f64;
use std::fmt;
use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::{CosinePDF, EmptyPDF, SpherePDF, PDF};
use crate::ray::Ray;
use crate::texture::{Texture, Solid};
use crate::utilities;
use crate::vec3::{Point3f, Vec3f};


pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn PDF>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray
}

pub trait Material: Send + Sync + fmt::Display {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _ray_in: &Ray, _rec: &HitRecord, _uv: (f64, f64), _point: &Point3f) -> Color {
        Color::ZERO
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}


pub struct Empty;

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "Material Empty.")
    }
}

impl Material for Empty {}


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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_rec: ScatterRecord = ScatterRecord { 
            attenuation: self.texture.value(rec.uv, &rec.point), 
            pdf_ptr: Arc::new(CosinePDF::new(&rec.normal)), 
            skip_pdf: false, 
            skip_pdf_ray: Ray::ZERO
        };
        Some(scatter_rec)
    }

    fn scattering_pdf(&self, _ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta: f64 = Vec3f::dot(&rec.normal, &Vec3f::unit_vector(scattered.direction()));
        if cos_theta < 0.0 {
            return 0.0;
        }
        cos_theta / f64::consts::PI
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected: Vec3f = Vec3f::reflect(ray_in.direction(), &rec.normal);
        reflected = Vec3f::unit_vector(&reflected) + self.fuzz * Vec3f::random_unit_vector();
        
        let scatter_rec: ScatterRecord = ScatterRecord { 
            attenuation: self.albedo, 
            pdf_ptr: Arc::new(EmptyPDF), 
            skip_pdf: true, 
            skip_pdf_ray: Ray::with_time(&rec.point, &reflected, ray_in.time())
        };
        Some(scatter_rec)
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
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

        let scatter_rec: ScatterRecord = ScatterRecord { 
            attenuation: Color::ONE, 
            pdf_ptr: Arc::new(EmptyPDF), 
            skip_pdf: true, 
            skip_pdf_ray: Ray::with_time(&rec.point, &direction, ray_in.time())
        };
        Some(scatter_rec)
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
    fn emitted(&self, _ray_in: &Ray, rec: &HitRecord, uv: (f64, f64), point: &Point3f) -> Color {
        if !rec.front_face {
            return Color::ZERO;
        }
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_rec: ScatterRecord = ScatterRecord { 
            attenuation: self.texture.value(rec.uv, &rec.point), 
            pdf_ptr: Arc::new(SpherePDF), 
            skip_pdf: false, 
            skip_pdf_ray: Ray::ZERO
        };
        Some(scatter_rec)
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * f64::consts::PI)
    }
}