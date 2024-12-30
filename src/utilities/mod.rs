use core::f64;
use rand::Rng;

#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}

#[inline(always)]
pub fn random() -> f64 {
    return rand::thread_rng().gen_range(0.0..1.0);
}

#[inline(always)]
pub fn random_range(min: f64, max: f64) -> f64 {
    return rand::thread_rng().gen_range(min..max);
}