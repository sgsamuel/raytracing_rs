use core::f64;
use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::{BasisAxis, ONB};
use crate::utilities;
use crate::vec3::{Point3f, Vec3f};

pub trait PDF: Send + Sync {
    fn value(&self, _direction: &Vec3f) -> f64 {
        0.0
    }

    fn generate(&self) -> Vec3f {
        Vec3f::ZERO
    }
}


pub struct EmptyPDF;

impl PDF for EmptyPDF {}


pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3f) -> f64 {
        1.0 / (4.0 * f64::consts::PI)
    }

    fn generate(&self) -> Vec3f {
        Vec3f::random_unit_vector()
    }
}


pub struct CosinePDF {
    uvw: ONB
}

impl CosinePDF {
    pub fn new(w: &Vec3f) -> Self {
        Self { uvw: ONB::new(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3f) -> f64 {
        let cosine_theta: f64 = Vec3f::dot(&Vec3f::unit_vector(direction), self.uvw.component(BasisAxis::W));
        f64::max(0.0, cosine_theta / f64::consts::PI)
    }

    fn generate(&self) -> Vec3f {
        self.uvw.transform(&Vec3f::random_cosine_direction())
    }
}


pub struct HittablePDF {
    objects: Arc<dyn Hittable>,
    origin: Point3f
}

impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable>, origin: &Point3f) -> Self {
        Self { objects, origin: *origin }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3f) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3f {
        self.objects.random(&self.origin)
    }
}


pub struct MixturePDF {
    light_pdf: Arc<dyn PDF>,
    surface_pdf: Arc<dyn PDF>
}

impl MixturePDF {
    pub fn new(light_pdf: Arc<dyn PDF>, surface_pdf: Arc<dyn PDF>) -> Self {
        Self { light_pdf, surface_pdf }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3f) -> f64 {
        0.5 * self.light_pdf.value(direction) + 0.5 * self.surface_pdf.value(direction)
    }

    fn generate(&self) -> Vec3f {
        if utilities::random() < 0.5 {
            self.light_pdf.generate()
        }
        else {
            self.surface_pdf.generate()
        }
    }
}