use std::fmt;
use std::sync::Arc;

use super::color::Color;
use super::vec3::{Axis, Point3};

pub trait Texture: Send + Sync + fmt::Display {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}


pub struct SolidTexture {
    albedo: Color
}

impl SolidTexture {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(&Color::new(r, g, b))
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

impl fmt::Display for SolidTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solid Texture Albedo: {}", self.albedo)
    }
}


pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { inv_scale: 1.0 / scale, even, odd }
    }

    pub fn from_color(scale: f64, color1: &Color, color2: &Color) -> Self {
        Self::new(
            scale, 
            Arc::new(SolidTexture::new(color1)),
            Arc::new(SolidTexture::new(color2))
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int: i32 = (self.inv_scale * p.component(Axis::X)).floor() as i32;
        let y_int: i32 = (self.inv_scale * p.component(Axis::Y)).floor() as i32;
        let z_int: i32 = (self.inv_scale * p.component(Axis::Z)).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            return self.even.value(u, v, p);
        }
        self.odd.value(u, v, p)
    }
}

impl fmt::Display for CheckerTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Checker Texture InvScale: {}; Even: {}; Odd: {}", self.inv_scale, self.even, self.odd)
    }
}