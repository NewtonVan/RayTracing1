#![allow(dead_code)]
use std::f32::{MAX as f32_MAX, MIN as f32_MIN};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::ray::Interval;
use crate::rtweekend::{random_double, random_double_in_range};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of range"),
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn vec_max() -> Vec3 {
        Vec3 {
            x: f32_MAX,
            y: f32_MAX,
            z: f32_MAX,
        }
    }

    pub fn vec_min() -> Vec3 {
        Vec3 {
            x: f32_MIN,
            y: f32_MIN,
            z: f32_MIN,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    fn linear_to_gamma(x: f32) -> f32 {
        if x > 0.0 {
            x.sqrt()
        } else {
            0.0
        }
    }

    pub fn rgba(&self) -> image::Rgba<u8> {
        let intensity = Interval::new(0.0, 0.999);
        image::Rgba([
            (intensity.clamp(Self::linear_to_gamma(self.x)) * 255.99) as u8,
            (intensity.clamp(Self::linear_to_gamma(self.y)) * 255.99) as u8,
            (intensity.clamp(Self::linear_to_gamma(self.z)) * 255.99) as u8,
            255,
        ])
    }

    pub fn elemul(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_double_in_range(min, max),
            y: random_double_in_range(min, max),
            z: random_double_in_range(min, max),
        }
    }

    pub fn random_unit_vec() -> Vec3 {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            let lensq = p.squared_length();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vec();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Vec3;

    #[test]
    fn test_eq() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(1.0, -2.0, 0.0));
        assert_ne!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(5.0, -3.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(-3.0, 7.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) * 5.0, Vec3::new(5.0, -10.0, 0.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-2.0, 2.0, 3.0)),
            -2.0 + 4.0 + 9.0
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        )
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(2.0, -2.0, 1.0).length(), 3.0);
    }

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(2.0, -2.0, 1.0).squared_length(), 9.0);
    }

    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3::new(2.0, -2.0, 1.0).unit(),
            Vec3::new(2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0)
        );
    }

    #[test]
    fn test_rgba() {
        assert_eq!(
            Vec3::new(0.0, 1.0, 0.5).rgba(),
            image::Rgba([0 as u8, 255 as u8, 127 as u8, 255 as u8])
        );
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_index() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec[0] = 2.0;
        vec[1] = 3.0;
        vec[2] = 0.0;
        assert_eq!(vec, Vec3::new(2.0, 3.0, 0.0));
    }

    #[test]
    fn test_index_panic() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = std::panic::catch_unwind(|| vec[5]);
        assert!(result.is_err());
    }
}
