use rand::thread_rng;
use rand::prelude::SliceRandom;
use rayon::prelude::*;

use crate::vec3::{Axis, Point3f, Vec3f};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PerlinTexture {
    Normal,
    Turbulence(u32),
    Marble(u32),
}

#[derive(Debug, Clone)]
pub struct Perlin {
    point_count: usize,
    random_vecs: Vec<Vec3f>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let random_vecs: Vec<Vec3f> = (0..point_count).into_par_iter().map(
            |_| {
                Vec3f::random_unit_vector()
            }
        ).collect::<Vec<Vec3f>>();

        let mut perm_x: Vec<usize> = (0..point_count).collect();
        perm_x.shuffle(&mut thread_rng());
        let mut perm_y: Vec<usize> = (0..point_count).collect();
        perm_y.shuffle(&mut  thread_rng());
        let mut perm_z: Vec<usize> = (0..point_count).collect();
        perm_z.shuffle(&mut  thread_rng());

        Self { point_count, random_vecs, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, point: &Point3f) -> f64 {
        let u: f64 = point.component(Axis::X) - point.component(Axis::X).floor();
        let v: f64 = point.component(Axis::Y) - point.component(Axis::Y).floor();
        let w: f64 = point.component(Axis::Z) - point.component(Axis::Z) .floor();

        let i: isize = point.component(Axis::X).floor() as isize;
        let j: isize = point.component(Axis::Y).floor() as isize;
        let k: isize = point.component(Axis::Z).floor() as isize;
        let mut c: [[[Vec3f; 2]; 2]; 2] = Default::default();

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let xi: usize = ((i + di as isize) & (self.point_count - 1) as isize) as usize;
                    let yi: usize = ((j + dj as isize) & (self.point_count - 1) as isize) as usize;
                    let zi: usize = ((k + dk as isize) & (self.point_count - 1) as isize) as usize;
                    let index: usize = self.perm_x[xi] ^ self.perm_y[yi] ^ self.perm_z[zi];
                    c[di][dj][dk] = self.random_vecs[index];
                }
            }
        }

        Self::trilinear_interp(&c, (u, v, w))
    }

    pub fn turbulence(&self, point: &Point3f, depth: u32) -> f64 {
        let mut accum: f64 = 0.0;
        let mut weight: f64 = 1.0;
        let mut temp_p = *point;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn trilinear_interp(c: &[[[Vec3f; 2]; 2]; 2], uvw: (f64, f64, f64)) -> f64 {
        let uu = uvw.0 * uvw.0 * (3.0 - 2.0 * uvw.0);
        let vv = uvw.1 * uvw.1 * (3.0 - 2.0 * uvw.1);
        let ww = uvw.2 * uvw.2 * (3.0 - 2.0 * uvw.2);
        let mut accum: f64 = 0.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vec: Vec3f = Vec3f::new(uvw.0 - i as f64, uvw.1 - j as f64, uvw.2 - k as f64);
                    accum += (i as f64).mul_add(uu, ((1 - i) as f64) * (1.0 - uu))
                           * (j as f64).mul_add(vv, ((1 - j) as f64) * (1.0 - vv))
                           * (k as f64).mul_add(ww, ((1 - k) as f64) * (1.0 - ww))
                           * Vec3f::dot(&c[i][j][k], &weight_vec);
                }
            }
        }
        accum
    }
}