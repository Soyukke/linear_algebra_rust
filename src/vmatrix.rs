use std::ops::{Add, Mul, AddAssign, Index, IndexMut};
use std::error::Error;
// クレート内のモジュールへのアクセスはcrate::で行う。
use crate::vector::*;
use crate::basic_trait::{One, Transpose};

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug, Clone)]
pub enum VMatrixError {
    UndefinedError(String),
}

#[derive(Debug, Clone)]
pub struct VMatrix<T> {
    pub rows: usize,
    pub cols: usize,
    //pub data: Vec<Vec<T>>,
    pub data: Vec<T>,
}

impl<T:Default+Clone> VMatrix<T> {
    pub fn new(rows: usize, cols: usize, init_val: T) -> VMatrix<T>
    where
        T: Clone,
    {
        VMatrix {
            rows,
            cols,
            //data: vec![vec![init_val; cols]; rows],
            data: vec![init_val; cols*rows],
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, T::default())
    }

}

impl<T:Default+Clone+One> VMatrix<T>
{
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, T::one())
    }
}

impl<T:Default+Clone> VMatrix<T>
where Standard: Distribution<T>
{
    pub fn rand(rows: usize, cols: usize) -> Self {
        let mut result = Self::new(rows, cols, T::default());
        let mut rng = rand::thread_rng();
        for i in 0..rows {
            for j in 0..cols {
                //result.data[i][j] =  rng.gen();
                result[(i, j)] =  rng.gen();
            }
        }
        result
    }
}

use std::fmt;
// 出力
impl<T: fmt::Display> fmt::Display for VMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{}x{} Matrix\n", self.rows, self.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                s += &format!("{:>5} ", self[(i, j)]);
            }
        s += &format!("\n");
        }
        write!(f, "{}", s)
    }
}


/// 行列積
/// (self.rows, self.cols) * (other.rows, other.cols)
/// require: self.cols == other.rows
impl<T: Mul<Output = T> + AddAssign + Default + Copy> Mul<VMatrix<T>> for VMatrix<T> {
    type Output = Result<Self, VMatrixError>;
    fn mul(self, other: Self) -> Self::Output {
        if self.cols != other.rows {
            return Err(VMatrixError::UndefinedError("matrix size does not match.".to_string()));
        }
        let mut result = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                let mut x = T::default();
                for k in 0..other.cols{
                    x += self[(i, k)] * other[(k, j)]
                }
                result[(i, j)] = x;
            }
        }
        Ok(result)
    }
}

/// 転置
impl<T: Default + Copy> Transpose for VMatrix<T> {
    type Output = VMatrix<T>;
    fn transpose(self) -> Self::Output {
        let mut result = VMatrix::<T>::zeros(self.cols, self.rows);
        for i in 0..self.rows{
            for j in 0..self.cols{
                //result.data[j*self.cols + i] = self.data[i*self.rows + j];
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }
}


/// Row-Major like Z
impl<T> std::ops::Index<(usize, usize)> for VMatrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row*self.cols+ col]
    }
}

/// Row-Major like Z
impl<T> std::ops::IndexMut<(usize, usize)> for VMatrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row*self.cols+ col]
    }
}
