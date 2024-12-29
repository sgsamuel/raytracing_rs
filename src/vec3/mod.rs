use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn component(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        return Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        };
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

macro_rules! impl_unary_op {
    ($VecType:ident $Op:ident $op_fn:ident $op_sym:tt) => {
        // v1 = &Vec3
        impl<'v1> $Op for &'v1 $VecType {
            type Output = $VecType;

            fn $op_fn(self) -> Vec3 {
                $VecType {
                  x: $op_sym self.x,
                  y: $op_sym self.y,
                  z: $op_sym self.z,
                }
            }
        }

        // v1 = Vec3
        impl $Op for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self) -> Vec3 {
              $op_sym &self
            }
        }
    }
}

macro_rules! impl_binary_op {
    ($VecType:ident $Op:ident $op_fn:ident $op_sym:tt) => {
        // v1: &Vec3, v2: &Vec3
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

        // v1: Vec3, v2: Vec3
        impl $Op<$VecType> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: $VecType) -> $VecType {
              &self $op_sym &other
            }
          }
      
        // v1: Vec3, v2: &Vec3
        impl<'v1> $Op<&'v1 $VecType> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: &'v1 $VecType) -> $VecType {
              &self $op_sym other
            }
        }
      
        // v1: &Vec3, v2: Vec3
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
        // v: &Vec3, c: f64
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
      
        // v: Vec3, c: f64
        impl $Op<f64> for $VecType {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: f64) -> $VecType {
              &self $op_sym other
            }
        }
      
        // c: f64, v: Vec3
        impl $Op<$VecType> for f64 {
            type Output = $VecType;
      
            #[inline]
            fn $op_fn(self, other: $VecType) -> $VecType {
              &other $op_sym self
            }
        }
        
        // c: f64, v: &Vec3
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
        // v = &Vec3
        impl<'v> $OpAssign<&'v $VecType> for $VecType {
            fn $op_fn(&mut self, other: &'v $VecType) {
                *self = $VecType {
                    x: self.x $op_sym other.x,
                    y: self.y $op_sym other.y,
                    z: self.z $op_sym other.z,
                };
            }
        }
  
        // v = Vec3
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


impl_unary_op!(Vec3 Neg neg -);

impl_binary_op!(Vec3 Add add +);
impl_binary_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_op!(Vec3 Sub sub -);
impl_binary_op_assign!(Vec3 SubAssign sub_assign -);

impl_binary_op!(Vec3 Mul mul *);
impl_float_op!(Vec3 Mul mul *);
impl_float_op_assign!(Vec3 MulAssign mul_assign *);

impl_float_op!(Vec3 Div div /);
impl_float_op_assign!(Vec3 DivAssign div_assign /);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component() {
        let v = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(v.component(Axis::X), v.x);
        assert_eq!(v.component(Axis::Y), v.y);
        assert_eq!(v.component(Axis::Z), v.z);
    }

    #[test]
    fn length() {
        let v1 = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(v1.length(), ((3.0 * 3.0 + 2.0 * 2.0 + 1.0 * 1.0) as f64).sqrt());

        let v2 = Vec3::ZERO;
        assert_eq!(v2.length(), 0.0);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::new(3.0, 2.0, 1.0);
        let len = v.length();
        assert!((v.unit_vector().length() - 1.0).abs() < 0.01);
        assert_eq!(v.unit_vector(), v / len);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(2.0, 3.0, 5.0);
        let v2 = Vec3::new(7.0, 11.0, 13.0);
        assert_eq!(Vec3::dot(&v1, &v2), 2.0 * 7.0 + 3.0 * 11.0 + 5.0 * 13.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::cross(&v1, &v2), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn neg() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(-&v, Vec3::new(0.0, -1.0, -2.0));
        assert_eq!(-v, Vec3::new(0.0, -1.0, -2.0));
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&v1 + &v2, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(v1 + &v2, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&v1 + v2, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(v1 + v2, Vec3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn add_assign() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);

        {
            let mut v2 = Vec3::ONE;
            v2 += v1;
            assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
        }

        {
            let mut v2 = Vec3::ONE;
            v2 += &v1;
            assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
        }
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&v1 - &v2, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 - &v2, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(&v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
    }
    
    #[test]
    fn sub_assign() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);

        {
            let mut v2 = Vec3::ONE;
            v2 -= v1;
            assert_eq!(v2, Vec3::new(1.0, 0.0, -1.0));
        }

        {
            let mut v2 = Vec3::ONE;
            v2 -= &v1;
            assert_eq!(v2, Vec3::new(1.0, 0.0, -1.0));
        }
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        let c = 3.5;
        assert_eq!(&v1 * &v2, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(v1 * &v2, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&v1 * v2, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(v1 * v2, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&v1 * c, Vec3::new(0.0, 3.5, 7.0));
        assert_eq!(v1 * c, Vec3::new(0.0, 3.5, 7.0));
        assert_eq!(c * &v1, Vec3::new(0.0, 3.5, 7.0));
        assert_eq!(c * v1, Vec3::new(0.0, 3.5, 7.0));
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::ONE;
        let c = 2.0;
        v *= c;
        assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn div() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        let c = 2.0;
        assert_eq!(&v / c, Vec3::new(0.0, 0.5, 1.0));
        assert_eq!(v / c, Vec3::new(0.0, 0.5, 1.0));
        assert_eq!(c / &v, Vec3::new(0.0, 0.5, 1.0));
        assert_eq!(c / v, Vec3::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::ONE;
        let c = 2.0;
        v /= c;
        assert_eq!(v, Vec3::new(0.5, 0.5, 0.5));
    }
}