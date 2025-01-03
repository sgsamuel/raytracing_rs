use crate::interval::Interval;
use crate::vec3::Axis;

pub type Color = crate::vec3::Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(pixel_color: Color) -> String {
    let r: f64 = linear_to_gamma(pixel_color.component(Axis::X));
    let g: f64 = linear_to_gamma(pixel_color.component(Axis::Y));
    let b: f64 = linear_to_gamma(pixel_color.component(Axis::Z));

    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte: u8 = (256.0 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (256.0 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (256.0 * intensity.clamp(b)) as u8;

    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}