use std::ops::{Add, Mul, AddAssign, Index, IndexMut};
// クレート内のモジュールへのアクセスはcrate::で行う。
use crate::vector::*;
use crate::basic_trait::{One, Transpose};

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug, Clone)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        let data = [[T::default(); COLS]; ROWS];
        Matrix {data}
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    pub fn nrow(&self) -> usize {
        ROWS
    }

    pub fn ncol(&self) -> usize {
        COLS 
    }
}

impl<T: Add<Output = T> + Default + One + Copy, const ROWS: usize, const COLS: usize> One for Matrix<T, ROWS, COLS> {
    fn one() -> Self {
        let mut data = [[T::default(); COLS]; ROWS];
        let minvalue = if ROWS < COLS {ROWS} else {COLS};
        for i in 0..minvalue {
                data[i][i] = T::one();
        }
        Self {data}
    }
}



impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut [T; COLS] {
        &mut self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        result
    }
}

// Matrix * Matrix = Matrix
// Tはmulできて、addAssignできて、コピーできて、デフォルトがあるtype
impl<T: Mul<Output = T> + AddAssign + Default + Copy, const ROWS: usize, const COLS: usize> Mul<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                //result.data[i][j] = self.data[i][j] * other.data[i][j];
                let mut x = T::default();
                for k in 0..ROWS {
                    x += self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>> for Matrix<T, ROWS, COLS> {
    type Output = Vector<T, ROWS>;
    fn mul(self, other: Vector<T, COLS>) -> Vector<T, ROWS> {
        // (ROWS, COLS) * (COLS, 1) = (ROWS, 1)
        let mut result = Vector::<T, ROWS>::new();

        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x += self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy, const ROWS: usize, const COLS: usize> Mul<T> for Matrix<T, ROWS, COLS> {
    type Output = Self;
    fn mul(self, other: T) -> Matrix<T, ROWS, COLS> {
        // (ROWS, COLS) * (COLS, 1) = (ROWS, 1)
        let mut result = Matrix::<T, ROWS, COLS>::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] * other;
            }
        }
        result
    }
}

/// random matrix
impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Distribution<Matrix<T, ROWS, COLS>> for Standard
where Standard: Distribution<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, ROWS, COLS> {
        let mut result = Matrix::<T, ROWS, COLS>::default();
        for i in 0..ROWS {
            for j in 0..COLS {
                let x: T = rng.gen();
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Transpose for Matrix<T, ROWS, COLS> {
    type Output = Matrix<T, COLS, ROWS>;
    fn transpose(self) -> Self::Output {
        let mut result = Matrix::<T, COLS, ROWS>::default();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}


use std::fmt;
// 出力
impl<T: fmt::Display, const ROWS: usize, const COLS: usize> fmt::Display for Matrix<T, ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{}x{} Matrix\n", ROWS, COLS);
        for i in 0..ROWS {
            for j in 0..COLS {
                s += &format!("{:>5} ", self.data[i][j]);
            }
        s += &format!("\n");
        }
        write!(f, "{}", s)
    }
}

