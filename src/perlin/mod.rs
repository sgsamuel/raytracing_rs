use rand::thread_rng;
use rand::prelude::SliceRandom;
use rayon::prelude::*;

use crate::utilities;
use crate::vec3::{Axis, Point3};

#[derive(Debug, Clone)]
pub struct Perlin {
    point_count: usize,
    random_values: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let random_values: Vec<f64> = (0..point_count).into_par_iter().map(
            |_| {
                utilities::random()
            }
        ).collect::<Vec<f64>>();

        let mut perm_x: Vec<usize> = (0..point_count).collect();
        perm_x.shuffle(&mut thread_rng());
        let mut perm_y: Vec<usize> = (0..point_count).collect();
        perm_y.shuffle(&mut  thread_rng());
        let mut perm_z: Vec<usize> = (0..point_count).collect();
        perm_z.shuffle(&mut  thread_rng());

        Self { point_count, random_values, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let x_index: usize = (((4.0 * point.component(Axis::X)) as isize) & (self.point_count - 1) as isize) as usize;
        let y_index: usize = (((4.0 * point.component(Axis::Y)) as isize) & (self.point_count - 1) as isize) as usize;
        let z_index: usize = (((4.0 * point.component(Axis::Z)) as isize) & (self.point_count - 1) as isize) as usize;
    
        self.random_values[self.perm_x[x_index] ^ self.perm_y[y_index] ^ self.perm_z[z_index]]
    }
}