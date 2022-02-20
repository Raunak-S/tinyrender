use std::ops::{Add, BitXor, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2D<T>
where
    T: num::Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T>
where
    T: num::Num,
{
    pub fn new() -> Self {
        Self {
            x: num::zero(),
            y: num::zero(),
        }
    }
    pub fn new_args(newx: T, newy: T) -> Self {
        Self { x: newx, y: newy }
    }
}

impl<T> Add for Vec2D<T>
where
    T: num::Num,
{
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<T> Sub for Vec2D<T>
where
    T: num::Num,
{
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl<T> Mul<f32> for Vec2D<T>
where
    T: num::Num + num::NumCast + Lossyf32,
{
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self::Output {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap() * _rhs),
            y: T::lossy_from_f32(self.x.to_f32().unwrap() * _rhs),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3D<T>
where
    T: num::Num,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3D<T>
where
    T: num::Num + num::ToPrimitive + num::NumCast + Copy + Lossyf32,
{
    pub fn new() -> Self {
        Self {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
        }
    }
    pub fn new_args(newx: T, newy: T, newz: T) -> Self {
        Self {
            x: newx,
            y: newy,
            z: newz,
        }
    }
    pub fn from_matrix(m: Matrix) -> Vec3f {
        Vec3f::new_args(m[0][0] / m[3][0], m[1][0] / m[3][0], m[2][0] / m[3][0])
    }
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
            .to_f32()
            .unwrap()
            .sqrt()
    }
    pub fn normalize(&mut self) -> &Self {
        *self = (*self) * (1.0 / self.norm());
        self
    }
    pub fn into_float(v: Vec3i) -> Vec3f {
        Vec3f {
            x: v.x as f32,
            y: v.y as f32,
            z: v.z as f32,
        }
    }
    pub fn into_int(v: Vec3f) -> Vec3i {
        Vec3i {
            x: v.x as i32,
            y: v.y as i32,
            z: v.z as i32,
        }
    }
}

impl<T> Add for Vec3D<T>
where
    T: num::Num,
{
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Add<Vec3D<f32>> for Vec3D<i32> {
    type Output = Self;

    fn add(self, rhs: Vec3D<f32>) -> Self::Output {
        Self {
            x: (self.x as f32 + rhs.x) as i32,
            y: (self.y as f32 + rhs.y) as i32,
            z: (self.z as f32 + rhs.z) as i32,
        }
    }
}

impl<T> Sub for Vec3D<T>
where
    T: num::Num,
{
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T> Mul<f32> for Vec3D<T>
where
    T: num::Num + num::NumCast + Lossyf32,
{
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self::Output {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap() * _rhs),
            y: T::lossy_from_f32(self.y.to_f32().unwrap() * _rhs),
            z: T::lossy_from_f32(self.z.to_f32().unwrap() * _rhs),
        }
    }
}

impl<T> Mul for Vec3D<T>
where
    T: num::Num + num::NumCast,
{
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x + self.y * rhs.y + self.z * rhs.z)
            .to_f32()
            .unwrap()
    }
}

impl<T> BitXor for Vec3D<T>
where
    T: num::Num + Copy,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> Index<usize> for Vec3D<T>
where
    T: num::Num,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        if index == 0 {
            &self.x
        } else {
            if index == 1 {
                &self.y
            } else {
                &self.z
            }
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

const DEFAULT_ALLOC: usize = 4;

#[derive(Clone)]
pub struct Matrix {
    m: Vec<Vec<f32>>,
    rows: i32,
    cols: i32,
}

impl Matrix {
    pub fn new(r: Option<i32>, c: Option<i32>) -> Self {
        let col_val = if let Some(num) = c {
            num as usize
        } else {
            DEFAULT_ALLOC
        };
        let row_val = if let Some(num) = r {
            num as usize
        } else {
            DEFAULT_ALLOC
        };
        Self {
            m: vec![vec![0.0; col_val]; row_val],
            rows: row_val as i32,
            cols: col_val as i32,
        }
    }
    pub fn from_vec3f(v: Vec3f) -> Matrix {
        let mut m = Matrix::new(Some(4), Some(1));
        m[0][0] = v.x;
        m[1][0] = v.y;
        m[2][0] = v.z;
        m[3][0] = 1.;
        m
    }
    pub fn nrows(&self) -> i32 {
        self.rows
    }
    pub fn ncols(&self) -> i32 {
        self.cols
    }
    pub fn identity(dimensions: i32) -> Self {
        let mut E = Matrix::new(Some(dimensions), Some(dimensions));
        for i in 0..dimensions as usize {
            for j in 0..dimensions as usize {
                E[i][j] = if i == j { 1. } else { 0. };
            }
        }
        E
    }
    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(Some(self.cols), Some(self.rows));
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                result[j][i] = self.m[i][j];
            }
        }
        result
    }
    pub fn inverse(&self) -> Self {
        assert!(self.rows == self.cols);
        let mut result = Matrix::new(Some(self.rows), Some(self.cols * 2));
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                result[i][j] = self.m[i][j];
            }
        }
        for i in 0..self.rows as usize {
            result[i][i + self.cols as usize] = 1.;
        }
        for i in 0..(self.rows - 1) as usize {
            for j in (0..=result.cols - 1).rev() {
                result[i][j as usize] /= result[i][i];
            }
            for k in i + 1..self.rows as usize {
                let coeff = result[k][i];
                for j in 0..result.cols as usize {
                    result[k][j] -= result[i][j] * coeff;
                }
            }
        }

        for j in (self.rows - 1..=result.cols - 1).rev() {
            result[(self.rows - 1) as usize][j as usize] /=
                result[(self.rows - 1) as usize][(self.rows - 1) as usize];
        }
        for i in (1..=(self.rows - 1) as usize).rev() {
            for k in (0..=i - 1).rev() {
                let coeff = result[k as usize][i as usize];
                for j in 0..result.cols as usize {
                    result[k as usize][j as usize] -= result[i as usize][j as usize] * coeff;
                }
            }
        }
        let mut truncate = Matrix::new(Some(self.rows), Some(self.cols));
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                truncate[i][j] = result[i][j + self.cols as usize];
            }
        }
        truncate
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows as usize);
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.rows as usize);
        &mut self.m[index]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows);
        let mut result = Matrix::new(Some(self.rows), Some(rhs.cols));
        for i in 0..self.rows as usize {
            for j in 0..rhs.cols as usize {
                result.m[i][j] = 0.;
                for k in 0..self.cols as usize {
                    result.m[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }
        result
    }
}
