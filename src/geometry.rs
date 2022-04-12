use std::ops::{Add, BitXor, Div, Index, IndexMut, Mul, Sub};

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
    T: num::Num + Copy,
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

    pub fn from_vec(v: &Vec<T>) -> Self {
        Self { x: v[0], y: v[1] }
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

impl<T: num::Num> Index<usize> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 2);
        if index == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

impl<T: num::Num> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 2);
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
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
    pub fn from_slice(s: &[f32; 3]) -> Vec3f {
        Vec3f::new_args(s[0], s[1], s[2])
    }
    pub fn from_vec(v: &Vec<T>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
    pub fn norm2(&self) -> f32 {
        *self * *self
    }
    pub fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }
    pub fn normalize(&mut self) -> &Self {
        *self = (*self) / self.norm();
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

impl<T: num::Num> Div<f32> for Vec3D<T>
where
    T: num::Num + num::NumCast + Lossyf32,
{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap() / rhs),
            y: T::lossy_from_f32(self.y.to_f32().unwrap() / rhs),
            z: T::lossy_from_f32(self.z.to_f32().unwrap() / rhs),
        }
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

impl<T: num::Num> IndexMut<usize> for Vec3D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);
        if index == 0 {
            &mut self.x
        } else {
            if index == 1 {
                &mut self.y
            } else {
                &mut self.z
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

pub struct dt;

impl dt {
    pub fn det(n: usize, src: &Matrix) -> f32 {
        if n == 1 && src.rows == 1 && src.cols == 1 {
            return src.m[0][0];
        }
        let mut ret = 0f32;
        for i in 0..n {
            ret += src.m[0][i] * src.cofactor(0, i as i32);
        }
        ret
    }
}

const DEFAULT_ALLOC: usize = 4;

#[derive(Clone, Debug)]
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
    pub fn col(&self, idx: i32) -> Vec<f32> {
        assert!(idx >= 0 && idx < self.cols);
        let mut ret = vec![];
        for i in 0..self.rows as usize {
            ret.push(self.m[i][idx as usize]);
        }
        ret
    }
    pub fn set_col(&mut self, idx: i32, v: &Vec<f32>) {
        assert!(idx < self.cols);
        for i in 0..self.rows as usize {
            self.m[i][idx as usize] = v[i];
        }
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
    pub fn det(&self) -> f32 {
        dt::det(self.cols as usize, self)
    }
    pub fn get_minor(&self, row: i32, col: i32) -> Matrix {
        let mut ret = Matrix::new(Some(self.rows - 1), Some(self.cols - 1));
        for i in 0..self.rows - 1 {
            for j in 0..self.cols - 1 {
                ret[i as usize][j as usize] = self.m
                    [if i < row { i as usize } else { i as usize + 1 }]
                    [if j < col { j as usize } else { j as usize + 1 }];
            }
        }
        ret
    }
    pub fn cofactor(&self, row: i32, col: i32) -> f32 {
        self.get_minor(row, col).det() * (if (row + col) % 2 != 0 { -1. } else { 1. })
    }
    pub fn adjugate(&self) -> Matrix {
        let mut ret = Matrix::new(Some(self.rows), Some(self.cols));
        for i in 0..self.rows {
            for j in 0..self.cols {
                ret[i as usize][j as usize] = self.cofactor(i, j);
            }
        }
        ret
    }
    pub fn invert_transpose(&self) -> Matrix {
        let ret = self.adjugate();
        let tmp = Vec3f::from_vec(&ret[0]) * Vec3f::from_vec(&self.m[0]);
        ret / tmp
    }
    pub fn invert(&self) -> Matrix {
        self.invert_transpose().transpose()
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

impl Mul<Vec4f> for Matrix {
    type Output = Vec4f;

    fn mul(self, rhs: Vec4f) -> Self::Output {
        let mut ret = Vec4f::new();
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                ret[i] += self[i][j] * rhs[j];
            }
        }
        ret
    }
}

impl Mul<Vec<f32>> for Matrix {
    type Output = Vec<f32>;

    fn mul(self, rhs: Vec<f32>) -> Self::Output {
        let mut ret = Vec::new();
        for i in 0..self.rows as usize {
            let mut ret_val = 0.;
            for j in 0..self.cols as usize {
                ret_val += self[i][j] * rhs[j];
            }
            ret.push(ret_val);
        }
        ret
    }
}

impl Div<f32> for Matrix {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut ret = Matrix::new(Some(self.rows), Some(self.cols));
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                ret[i][j] = self.m[i][j] / rhs;
            }
        }
        ret
    }
}

pub fn cross<T: num::Num + Copy>(v1: Vec3D<T>, v2: Vec3D<T>) -> Vec3D<T> {
    Vec3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn embed(v: &Vec3f, fill: Option<f32>) -> Vec4f {
    let fill = match fill {
        Some(val) => val,
        None => 1.,
    };
    let mut ret = Vec4f::new();
    for i in (0..4).rev() {
        ret[i] = if i < 3 { v[i] } else { fill };
    }
    ret
}

pub fn embed_refactor<const T: usize>(v: &Vec<f32>, fill: Option<f32>) -> Vec<f32> {
    let fill = match fill {
        Some(val) => val,
        None => 1.,
    };

    let mut ret: Vec<f32> = vec![];
    for i in 0..T {
        ret.push(if i < v.len() { v[i] } else { fill });
    }
    ret
}

pub fn proj(v: Vec4f) -> Vec2f {
    let mut ret = Vec2f::new();
    for i in (0..2).rev() {
        ret[i] = v[i];
    }
    ret
}

pub fn proj_refactor<T: Copy>(v: Vec<T>, len: usize) -> Vec<T> {
    let mut ret: Vec<T> = vec![];
    for i in 0..len {
        ret.push(v[i]);
    }
    ret
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4D<T>
where
    T: num::Num,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub a: T,
}

pub type Vec4f = Vec4D<f32>;

impl Vec4f {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            a: 0.,
        }
    }
}

impl<T: num::Num> Into<Vec<T>> for Vec4D<T> {
    fn into(self) -> Vec<T> {
        vec![self.x, self.y, self.z, self.a]
    }
}

impl<T: num::Num> Div<f32> for Vec4D<T>
where
    T: num::Num + num::NumCast + Lossyf32,
{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: T::lossy_from_f32(self.x.to_f32().unwrap() / rhs),
            y: T::lossy_from_f32(self.y.to_f32().unwrap() / rhs),
            z: T::lossy_from_f32(self.z.to_f32().unwrap() / rhs),
            a: T::lossy_from_f32(self.a.to_f32().unwrap() / rhs),
        }
    }
}

impl<T: num::Num> Index<usize> for Vec4D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 4);
        if index == 0 {
            &self.x
        } else {
            if index == 1 {
                &self.y
            } else {
                if index == 2 {
                    &self.z
                } else {
                    &self.a
                }
            }
        }
    }
}

impl<T: num::Num> IndexMut<usize> for Vec4D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 4);
        if index == 0 {
            &mut self.x
        } else {
            if index == 1 {
                &mut self.y
            } else {
                if index == 2 {
                    &mut self.z
                } else {
                    &mut self.a
                }
            }
        }
    }
}
