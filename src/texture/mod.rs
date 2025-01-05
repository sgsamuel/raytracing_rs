use std::fmt;
use std::path::Path;
use std::sync::Arc;

use image::{DynamicImage, GenericImageView};

use crate::color::Color;
use crate::interval::Interval;
use crate::perlin::{Perlin, PerlinTexture};
use crate::vec3::{Axis, Point3f};

pub trait Texture: Send + Sync + fmt::Display {
    fn value(&self, uv: (f64, f64), point: &Point3f) -> Color;
}


pub struct Solid {
    albedo: Color
}

impl fmt::Display for Solid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solid Texture Albedo: {}", self.albedo)
    }
}

impl Solid {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(&Color::new(r, g, b))
    }
}

impl Texture for Solid {
    fn value(&self, _uv: (f64, f64), _point: &Point3f) -> Color {
        self.albedo
    }
}


pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl fmt::Display for Checker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Checker Texture InvScale: {}; Even: {}; Odd: {}", self.inv_scale, self.even, self.odd)
    }
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { inv_scale: 1.0 / scale, even, odd }
    }

    pub fn from_color(scale: f64, color1: &Color, color2: &Color) -> Self {
        Self::new(
            scale, 
            Arc::new(Solid::new(color1)),
            Arc::new(Solid::new(color2))
        )
    }
}

impl Texture for Checker {
    fn value(&self, uv: (f64, f64), point: &Point3f) -> Color {
        let x_int: i32 = (self.inv_scale * point.component(Axis::X)).floor() as i32;
        let y_int: i32 = (self.inv_scale * point.component(Axis::Y)).floor() as i32;
        let z_int: i32 = (self.inv_scale * point.component(Axis::Z)).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            return self.even.value(uv, point);
        }
        self.odd.value(uv, point)
    }
}


pub struct Image {
    img: DynamicImage
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Image Texture")
    }
}

impl Image {
    pub fn read_image(filepath: &Path) -> Result<Self, String> {
        let img: DynamicImage = image::open(filepath).map_err(|err| err.to_string())?;
        Ok(Self { img })
    }
}

impl Texture for Image {
    fn value(&self, uv: (f64, f64), _point: &Point3f) -> Color {
        if self.img.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let clamped_u: f64 = Interval::UNIT.clamp(uv.0);
        let clamped_v: f64 = 1.0 - Interval::UNIT.clamp(uv.1);  // Flip V to image coordinates

        let x: u32 = (clamped_u * self.img.width() as f64) as u32;
        let y: u32 = (clamped_v * self.img.height() as f64) as u32;
        let pixel = self.img.get_pixel(x, y);

        let color_scale: f64 = 1.0 / 255.0;
        Color::new(
            color_scale * f64::from(pixel[0]), 
            color_scale * f64::from(pixel[1]), 
            color_scale * f64::from(pixel[2])
        )
    }
}


pub struct Noise {
    noise: Perlin,
    perlin_texture: PerlinTexture,
    scale: f64
}

impl fmt::Display for Noise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Noise Texture")
    }
}

impl Noise {
    pub fn new(point_count: usize, perlin_texture: PerlinTexture, scale: f64) -> Self {
        Self { noise: Perlin::new(point_count), perlin_texture, scale }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: (f64, f64), point: &Point3f) -> Color {
        let noise_factor: f64 = match self.perlin_texture {
            PerlinTexture::Normal => {
                0.5 * (1.0 + self.noise.noise(&(self.scale * point)))
            },
            PerlinTexture::Turbulence(depth) => {
                self.noise.turbulence(point, depth)
            },
            PerlinTexture::Marble(depth) => {
                let noise = self.noise.turbulence(point, depth);
                0.5 * (1.0 + f64::sin(self.scale.mul_add(point.component(Axis::Z), 10.0 * noise)))
            }
        };
        noise_factor * Color::ONE
    }
}