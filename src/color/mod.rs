use crate::interval::Interval;
use crate::vec3::Axis;

pub type Color = crate::vec3::Vec3f;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(pixel_color: Color) -> String {
    let mut r: f64 = linear_to_gamma(pixel_color.component(Axis::X));
    let mut g: f64 = linear_to_gamma(pixel_color.component(Axis::Y));
    let mut b: f64 = linear_to_gamma(pixel_color.component(Axis::Z));

    // Replace NaN components with zero.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte: u8 = (256.0 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (256.0 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (256.0 * intensity.clamp(b)) as u8;

    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}