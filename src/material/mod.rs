use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material {
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


pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &rec.normal) > 0.0
    }
}