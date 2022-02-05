use std::ops::{Add, Sub, Mul, BitXor};

#[derive(Debug, Clone, Copy)]
pub struct Vec2D<T> where T: num::Num {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> where T: num::Num {
    pub fn new() -> Self { Self { x: num::zero(), y: num::zero() } }
    pub fn new_args(newx: T, newy: T) -> Self { Self { x: newx, y: newy } }
}

impl<T> Add for Vec2D<T> where T: num::Num {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self { 
        Self { 
            x: self.x+_rhs.x, 
            y: self.y+_rhs.y, 
        } 
    }
}

impl<T> Sub for Vec2D<T> where T: num::Num {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x-_rhs.x,
            y: self.y-_rhs.y,
        }
    }
}

impl<T> Mul<f32> for Vec2D<T> where T: num::Num
                                        + num::NumCast
                                        + Lossyf32 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap()*_rhs),
            y: T::lossy_from_f32(self.x.to_f32().unwrap()*_rhs),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3D<T> where T: num::Num {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3D<T> where T: num::Num
                        + num::ToPrimitive
                        + num::NumCast
                        + Copy
                        + Lossyf32 {
    pub fn new() -> Self { Self { x: num::zero(), y: num::zero(), z: num::zero(), } }
    pub fn new_args(newx: T, newy: T, newz: T) -> Self { Self { x: newx, y: newy, z: newz, } }
    pub fn norm(&self) -> f32 { (self.x*self.x+self.y*self.y+self.z*self.z).to_f32().unwrap() }
    pub fn normalize(&mut self) {
        let inv_norm = 1.0/self.norm();
        *self = Self { x: T::lossy_from_f32(self.x.to_f32().unwrap()*inv_norm),
                       y: T::lossy_from_f32(self.y.to_f32().unwrap()*inv_norm),
                       z: T::lossy_from_f32(self.z.to_f32().unwrap()*inv_norm), }; 
    }
}

impl<T> Add for Vec3D<T> where T: num::Num {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self { 
        Self { 
            x: self.x+_rhs.x, 
            y: self.y+_rhs.y,
            z: self.z+_rhs.z, 
        } 
    }
}

impl<T> Sub for Vec3D<T> where T: num::Num {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x-_rhs.x,
            y: self.y-_rhs.y,
            z: self.z-_rhs.z,
        }
    }
}

impl<T> Mul<f32> for Vec3D<T> where T: num::Num
                                    + num::NumCast
                                    + Lossyf32 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap()*_rhs),
            y: T::lossy_from_f32(self.y.to_f32().unwrap()*_rhs),
            z: T::lossy_from_f32(self.z.to_f32().unwrap()*_rhs),
        }
    }
}

impl<T> Mul for Vec3D<T> where T: num::Num
                                + num::NumCast
                                + Lossyf32 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.x*rhs.x + self.y*rhs.y + self.z*rhs.z).to_f32().unwrap()
    }
}

impl<T> BitXor for Vec3D<T> where T: num::Num
                                    + Copy {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y*rhs.z-self.z*rhs.y,
            y: self.z*rhs.x-self.x*rhs.z,
            z: self.x*rhs.y-self.y*rhs.x,
        }
    }
}

pub type Vec2i = Vec2D<i32>;
pub type Vec3i = Vec3D<i32>;
pub type Vec2f = Vec2D<f32>;
pub type Vec3f = Vec3D<f32>;

pub trait Lossyf32: Sized {
    fn lossy_from_f32(f: f32) -> Self;
}

impl Lossyf32 for i32 {
    fn lossy_from_f32(f: f32) -> Self {
        f as Self
    }
}

impl Lossyf32 for f32 {
    fn lossy_from_f32(f: f32) -> Self {
        f
    }
}