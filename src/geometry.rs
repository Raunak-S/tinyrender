use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Vec2D<T> where T: num::Num {
    x: T,
    y: T,
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

impl<T> Mul<f64> for Vec2D<T> where T: num::Num
                                    + From<f64>
                                    + Mul<f64, Output = f64> {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: T::from(self.x*_rhs),
            y: T::from(self.y*_rhs),
        }
    }
}

pub type Vec2i = Vec2D<i32>;
pub type Vec2f = Vec2D<f32>;