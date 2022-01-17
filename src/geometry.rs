use std::ops::{Add, Sub, Mul, BitXor};

#[derive(Debug, Clone)]
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

impl<T, F> Mul<F> for Vec2D<T> where T: num::Num
                                    + From<F>
                                    + num::ToPrimitive,
                                    F: num::Float {
    type Output = Self;

    fn mul(self, _rhs: F) -> Self {
        Self {
            x: T::from(F::from(self.x).unwrap()*_rhs),
            y: T::from(F::from(self.y).unwrap()*_rhs),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vec3D<T> where T: num::Num {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3D<T> where T: num::Num {
    pub fn new() -> Self { Self { x: num::zero(), y: num::zero(), z: num::zero(), } }
    pub fn new_args(newx: T, newy: T, newz: T) -> Self { Self { x: newx, y: newy, z: newz, } }
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

impl<T, F> Mul<F> for Vec3D<T> where T: num::Num
                                    + From<F>
                                    + Mul<F, Output = F>,
                                    F: num::Float {
    type Output = Self;

    fn mul(self, _rhs: F) -> Self {
        Self {
            x: T::from(self.x*_rhs),
            y: T::from(self.y*_rhs),
            z: T::from(self.z*_rhs),
        }
    }
}

impl<T> BitXor for Vec3D<T> where T: num::Num {
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