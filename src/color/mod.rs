use super::interval::Interval;

pub type Color = super::vec3::Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte)
}