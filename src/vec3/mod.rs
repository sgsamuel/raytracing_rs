use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;
use std::slice::Iter;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::utilities;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn iterator() -> Iter<'static, Axis> {
        static AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXES.iter()
    }
}

impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0..=2) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

pub type Vec3f = Vec3<f64>;
pub type Point3f = Vec3<f64>;

impl<T> fmt::Display for Vec3<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Default for Vec3f {
    fn default() -> Self {
        Vec3f::ZERO
    }
}

impl<T> Vec3<T> where T: Copy + Clone {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn component(&self, axis: Axis) -> T {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

impl Vec3f {
    pub const ZERO: Vec3f = Vec3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3f = Vec3f {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const INFINITY: Vec3f = Vec3f {
        x: f64::INFINITY,
        y: f64::INFINITY,
        z: f64::INFINITY,
    };

    pub fn random() -> Vec3f {
        Vec3f::new(utilities::random(), utilities::random(), utilities::random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3f {
        Vec3f::new(
            utilities::random_f64_range(min, max), 
            utilities::random_f64_range(min, max), 
            utilities::random_f64_range(min, max)
        )
    }

    pub fn sample_unit_square() -> Vec3f {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3f::new(utilities::random() - 0.5, utilities::random() - 0.5, 0.0)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let eps: f64 = 1e-8;
        (self.x.abs() < eps) && (self.y.abs() < eps) && (self.z.abs() < eps)
    }

    #[inline]
    pub fn unit_vector(v: &Vec3f) -> Vec3f {
        v / v.length()
    }

    #[inline]
    pub fn random_unit_vector() -> Vec3f {
        loop {
            let p: Vec3f = Self::random_range(-1.0, 1.0);
            let lensq: f64 = p.length_squared();
            if f64::EPSILON < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vec3f {
        loop {
            let p: Vec3f = Vec3f::new(
                utilities::random_f64_range(-1.0, 1.0), 
                utilities::random_f64_range(-1.0, 1.0),
                0.0
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3f) -> Vec3f {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    #[inline]
    pub fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
        v - 2.0 * Self::dot(v, n) * n
    }

    #[inline]
    pub fn refract(uv: &Vec3f, n: &Vec3f, etai_over_etat: f64) -> Vec3f {
        let cos_theta: f64 = Self::dot(&-uv, n).min(1.0);
        let r_out_perp: Vec3f =  etai_over_etat * (uv + cos_theta*n);
        let r_out_parallel: Vec3f = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    #[inline]
    pub fn cross(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        Vec3f {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

macro_rules! impl_unary_op {
    ($VecType:ident $Op:ident $op_fn:ident $op_sym:tt) => {
        // v1 = &Vec3f
        impl<'v1> $Op for &'v1 $VecType {
            type Output = $VecType;

            fn $op_fn(self) -> $VecType {
                $VecType {
                  x: $op_sym self.x,
                  y: $op_sym self.y,
                  z: $op_sym self.z,
                }
            }
        }

        // v1 = Vec3f
        impl $Op for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self) -> $VecType {
              $op_sym &self
            }
        }
    }
}

macro_rules! impl_binary_op {
    ($VecType:ident $Op:ident $op_fn:ident $op_sym:tt) => {
        // v1: &Vec3f, v2: &Vec3f
        impl<'v1, 'v2> $Op<&'v1 $VecType> for &'v2 $VecType {
            type Output = $VecType;

            fn $op_fn(self, other: &'v1 $VecType) -> $VecType {
                $VecType {
                    x: self.x $op_sym other.x,
                    y: self.y $op_sym other.y,
                    z: self.z $op_sym other.z,
                }
            }
        }

        // v1: Vec3f, v2: Vec3f
        impl $Op<$VecType> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: $VecType) -> $VecType {
              &self $op_sym &other
            }
          }
      
        // v1: Vec3f, v2: &Vec3f
        impl<'v1> $Op<&'v1 $VecType> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: &'v1 $VecType) -> $VecType {
              &self $op_sym other
            }
        }
      
        // v1: &Vec3f, v2: Vec3f
        impl<'v1> $Op<$VecType> for &'v1 $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: $VecType) -> $VecType {
              self $op_sym &other
            }
        }
    }
}

macro_rules! impl_float_op {
    ($VecType:ident $Op:ident $op_fn:ident $op_sym:tt) => {
        // v: &Vec3f, c: f64
        impl<'v> $Op<f64> for &'v $VecType {
            type Output = $VecType;

            fn $op_fn(self, other: f64) -> $VecType {
              $VecType {
                x: self.x $op_sym other,
                y: self.y $op_sym other,
                z: self.z $op_sym other
              }
            }
        }
      
        // v: Vec3f, c: f64
        impl $Op<f64> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: f64) -> $VecType {
              &self $op_sym other
            }
        }
      
        // c: f64, v: Vec3f
        impl $Op<$VecType> for f64 {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: $VecType) -> $VecType {
              &other $op_sym self
            }
        }
        
        // c: f64, v: &Vec3f
        impl<'v1> $Op<&'v1 $VecType> for f64 {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: &'v1 $VecType) -> $VecType {
              other $op_sym self
            }
        }
    }
}

macro_rules! impl_binary_op_assign {
    ($VecType:ident $OpAssign:ident $op_fn:ident $op_sym:tt) => {
        // v = &Vec3f
        impl<'v> $OpAssign<&'v $VecType> for $VecType {

            fn $op_fn(&mut self, other: &'v $VecType) {
                *self = $VecType {
                    x: self.x $op_sym other.x,
                    y: self.y $op_sym other.y,
                    z: self.z $op_sym other.z,
                };
            }
        }
  
        // v = Vec3f
        impl $OpAssign for $VecType {
            #[inline]
            fn $op_fn(&mut self, other: $VecType) {
            *self = *self $op_sym &other
            }
        }
    };
}

macro_rules! impl_float_op_assign {
    ($VecType:ident $OpAssign:ident $op_fn:ident $op_sym:tt) => {
        impl<'v> $OpAssign<f64> for $VecType {

            fn $op_fn(&mut self, other: f64) {
                *self = $VecType {
                    x: self.x $op_sym other,
                    y: self.y $op_sym other,
                    z: self.z $op_sym other,
                };
            }
        }
    };
}


impl_unary_op!(Vec3f Neg neg -);

impl_binary_op!(Vec3f Add add +);
impl_binary_op_assign!(Vec3f AddAssign add_assign +);

impl_binary_op!(Vec3f Sub sub -);
impl_binary_op_assign!(Vec3f SubAssign sub_assign -);

impl_binary_op!(Vec3f Mul mul *);
impl_float_op!(Vec3f Mul mul *);
impl_float_op_assign!(Vec3f MulAssign mul_assign *);

impl_float_op!(Vec3f Div div /);
impl_float_op_assign!(Vec3f DivAssign div_assign /);


#[cfg(test)]
mod tests {
    use crate::vec3::*;

    #[test]
    fn component() {
        let v: Vec3f = Vec3f::new(3.0, 2.0, 1.0);
        assert_eq!(v.component(Axis::X), v.x);
        assert_eq!(v.component(Axis::Y), v.y);
        assert_eq!(v.component(Axis::Z), v.z);
    }

    #[test]
    fn length() {
        let v1: Vec3f = Vec3f::new(3.0, 2.0, 1.0);
        assert_eq!(v1.length(), ((3.0 * 3.0 + 2.0 * 2.0 + 1.0 * 1.0) as f64).sqrt());

        let v2: Vec3f = Vec3f::ZERO;
        assert_eq!(v2.length(), 0.0);
    }

    #[test]
    fn near_zero() {
        let v1: Vec3f = Vec3f::new(3.0, 2.0, 1.0);
        assert_eq!(v1.near_zero(), false);

        let v2: Vec3f = Vec3f::ZERO;
        assert_eq!(v2.near_zero(), true);

        let v3: Vec3f = Vec3f::new(0.0, 1.0, 0.0);
        assert_eq!(v3.near_zero(), false);
    }

    #[test]
    fn reflect() {
        let v1: Vec3f = Vec3f::new(3.0, 2.0, 1.0);
        let v2: Vec3f = Vec3f::ONE;
        assert_eq!(Vec3f::reflect(&v1, &v2), Vec3f::new(-9.0, -10.0, -11.0));
    }

    #[test]
    fn refract() {
        let uv: Vec3f = Vec3f::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0);
        let n: Vec3f = Vec3f::ONE;
        let etai_over_etat: f64 = 0.5;

        assert_eq!(
            Vec3f::refract(&uv, &n, etai_over_etat), 
            Vec3f::new(-0.9023689270621825, -0.7357022603955159, -0.7357022603955159)
        );
    }

    #[test]
    fn unit_vector() {
        let v: Vec3f = Vec3f::new(3.0, 2.0, 1.0);
        let len: f64 = v.length();
        assert!((Vec3f::unit_vector(&v).length() - 1.0).abs() < 0.01);
        assert_eq!(Vec3f::unit_vector(&v), v / len);
    }

    #[test]
    fn dot() {
        let v1: Vec3f = Vec3f::new(2.0, 3.0, 5.0);
        let v2: Vec3f = Vec3f::new(7.0, 11.0, 13.0);
        assert_eq!(Vec3f::dot(&v1, &v2), 2.0 * 7.0 + 3.0 * 11.0 + 5.0 * 13.0);
    }

    #[test]
    fn cross() {
        let v1: Vec3f = Vec3f::new(1.0, 0.0, 0.0);
        let v2: Vec3f = Vec3f::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3f::cross(&v1, &v2), Vec3f::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn neg() {
        let v: Vec3f = Vec3f::new(0.0, 1.0, 2.0);
        assert_eq!(-&v, Vec3f::new(0.0, -1.0, -2.0));
        assert_eq!(-v, Vec3f::new(0.0, -1.0, -2.0));
    }

    #[test]
    fn add() {
        let v1: Vec3f = Vec3f::new(0.0, 1.0, 2.0);
        let v2: Vec3f = Vec3f::new(3.0, 4.0, 5.0);
        assert_eq!(&v1 + &v2, Vec3f::new(3.0, 5.0, 7.0));
        assert_eq!(v1 + &v2, Vec3f::new(3.0, 5.0, 7.0));
        assert_eq!(&v1 + v2, Vec3f::new(3.0, 5.0, 7.0));
        assert_eq!(v1 + v2, Vec3f::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn add_assign() {
        let v1: Vec3f = Vec3f::new(0.0, 1.0, 2.0);

        {
            let mut v2: Vec3f = Vec3f::ONE;
            v2 += v1;
            assert_eq!(v2, Vec3f::new(1.0, 2.0, 3.0));
        }

        {
            let mut v2: Vec3f = Vec3f::ONE;
            v2 += &v1;
            assert_eq!(v2, Vec3f::new(1.0, 2.0, 3.0));
        }
    }

    #[test]
    fn sub() {
        let v1: Vec3f = Vec3f::new(0.0, 1.0, 2.0);
        let v2: Vec3f = Vec3f::new(3.0, 4.0, 5.0);
        assert_eq!(&v1 - &v2, Vec3f::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 - &v2, Vec3f::new(-3.0, -3.0, -3.0));
        assert_eq!(&v1 - v2, Vec3f::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 - v2, Vec3f::new(-3.0, -3.0, -3.0));
    }
    
    #[test]
    fn sub_assign() {
        let v1: Vec3f = Vec3f::new(0.0, 1.0, 2.0);

        {
            let mut v2: Vec3f = Vec3f::ONE;
            v2 -= v1;
            assert_eq!(v2, Vec3f::new(1.0, 0.0, -1.0));
        }

        {
            let mut v2: Vec3f = Vec3f::ONE;
            v2 -= &v1;
            assert_eq!(v2, Vec3f::new(1.0, 0.0, -1.0));
        }
    }

    #[test]
    fn mul() {
        let v1: Vec3f = Vec3f::new(0.0, 1.0, 2.0);
        let v2: Vec3f = Vec3f::new(3.0, 4.0, 5.0);
        let c: f64 = 3.5;
        assert_eq!(&v1 * &v2, Vec3f::new(0.0, 4.0, 10.0));
        assert_eq!(v1 * &v2, Vec3f::new(0.0, 4.0, 10.0));
        assert_eq!(&v1 * v2, Vec3f::new(0.0, 4.0, 10.0));
        assert_eq!(v1 * v2, Vec3f::new(0.0, 4.0, 10.0));
        assert_eq!(&v1 * c, Vec3f::new(0.0, 3.5, 7.0));
        assert_eq!(v1 * c, Vec3f::new(0.0, 3.5, 7.0));
        assert_eq!(c * &v1, Vec3f::new(0.0, 3.5, 7.0));
        assert_eq!(c * v1, Vec3f::new(0.0, 3.5, 7.0));
    }

    #[test]
    fn mul_assign() {
        let mut v: Vec3f = Vec3f::ONE;
        let c: f64 = 2.0;
        v *= c;
        assert_eq!(v, Vec3f::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn div() {
        let v: Vec3f = Vec3f::new(0.0, 1.0, 2.0);
        let c: f64 = 2.0;
        assert_eq!(&v / c, Vec3f::new(0.0, 0.5, 1.0));
        assert_eq!(v / c, Vec3f::new(0.0, 0.5, 1.0));
        assert_eq!(c / &v, Vec3f::new(0.0, 0.5, 1.0));
        assert_eq!(c / v, Vec3f::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn div_assign() {
        let mut v: Vec3f = Vec3f::ONE;
        let c: f64 = 2.0;
        v /= c;
        assert_eq!(v, Vec3f::new(0.5, 0.5, 0.5));
    }
}