use crate::basic_trait::{One, Transpose};
use std::ops::{Mul, Add, AddAssign};
use rand::Rng;
use rand::prelude::Distribution;
use rand::distributions::Standard;

#[derive(Debug, Clone)]
pub struct Array<T, const D: usize> {
    pub dims: [usize; D],
    pub data: Vec<T>,
}

pub type Vector<T> = Array<T, 1>;
pub type Matrix<T> = Array<T, 2>;

impl<T:Default+Clone, const D: usize> Array<T, D> {
    pub fn new(dims: [usize; D], init_val: T) -> Self
    {
        let len = dims.into_iter().reduce(|a, b| a*b).unwrap();
        Array {
            dims,
            data: vec![init_val; len],
        }
    }

    pub fn zeros(dims: [usize; D]) -> Self {
        Self::new(dims, T::default())
    }

}

impl<T:Default+Clone+One, const D: usize> Array<T, D>
{
    pub fn ones(dims: [usize; D]) -> Self {
        Self::new(dims, T::one())
    }

    pub fn identity(dims: [usize; D]) -> Self {
        let mut result = Self::zeros(dims);
        let n = *dims.iter().min().unwrap();
        for i in 0..n {
            let idx: [usize; D] = [i; D];
            result[idx] = T::one();
        } 
        result
    }
}

impl<T:Default+Clone, const D: usize> Array<T, D>
where Standard: Distribution<T>
{
    pub fn rand(dims: [usize; D]) -> Self {
        let mut result = Self::new(dims, T::default());
        let mut rng = rand::thread_rng();
        for i in 0..result.data.len() {
                result.data[i] =  rng.gen();
        }
        result
    }
}

impl<T, const D: usize> std::ops::Index<([usize; D])> for Array<T, D> {
    type Output = T;

    fn index(&self, dims: [usize; D]) -> &Self::Output {
        //dims[0] -> reduce(dims[1:end], *)
        //dims[1] -> reduce(dims[2:end], *)
        let n = dims.len();
        let mut idx = 0;
        let mut stride = 1;
        // x[n-1] + x[n-2]*D[n-1] + x[n-3]*D[n-2]*D[n-1] + ... + x[0]*D[1]*D[2]*...*D[n-1]
        for i in (0..n).rev() {
            idx += dims[i] * stride;
            stride *= self.dims[i];
        }
        &self.data[idx]
    }
}

// Row-Major like Z
impl<T, const D: usize> std::ops::IndexMut<[usize; D]> for Array<T, D> {
    fn index_mut(&mut self, dims: [usize; D]) -> &mut Self::Output {
        let n = dims.len();
        let mut idx = 0;
        let mut stride = 1;
        // x[n-1] + x[n-2]*D[n-1] + x[n-3]*D[n-2]*D[n-1] + ... + x[0]*D[1]*D[2]*...*D[n-1]
        for i in (0..n).rev() {
            idx += dims[i] * stride;
            stride *= self.dims[i];
        }
        &mut self.data[idx]
    }
}


use std::fmt;

/// # Examples
impl<T: fmt::Display> fmt::Display for Array<T, 1> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{} Array\n", self.dims[0]);
        let rows = self.dims[0];
        for i in 0..rows {
            s += &format!("{:8.5} ", self[[i]]);
            s += &format!("\n");
        }
        write!(f, "{}", s)
    }
}

/// # Examples
/// 3x3 Array
///     1     1     1 
///     1     1     1 
///     1     1     1 
impl<T: fmt::Display> fmt::Display for Array<T, 2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{}x{} Array\n", self.dims[0], self.dims[1]);
        let rows = self.dims[0];
        let cols = self.dims[1];
        for i in 0..rows {
            for j in 0..cols {
                s += &format!("{:8.5} ", self[[i, j]]);
            }
        s += &format!("\n");
        }
        write!(f, "{}", s)
    }
}

/// # Examples
/// 3x3x3 Array
/// [0, :, :]
///     2     2     2 
///     2     2     2 
///     2     2     2 
/// [1, :, :]
///     2     2     2 
///     2     2     2 
///     2     2     2 
/// [2, :, :]
///     2     2     2 
///     2     2     2 
///     2     2     2 
/// 
impl<T: fmt::Display> fmt::Display for Array<T, 3> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim0 = self.dims[0];
        let rows = self.dims[0];
        let cols = self.dims[1];
        let mut s = format!("{}x{}x{} Array\n", dim0, rows, cols);
        for a in 0..dim0 {
            s += &format!("[{}, :, :]\n", a);
            for i in 0..rows {
                for j in 0..cols {
                    s += &format!("{:8.5} ", self[[a, i, j]]);
                }
                s += &format!("\n");
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: Add<Output = T> + AddAssign + Default + Copy> Add<Array<T, 1>> for Array<T, 1> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let self_rows = self.dims[0];
        let other_rows = other.dims[0];
        if self_rows != other_rows {
            panic!("matrix size does not match.");
        }
        let mut result = Self::zeros([self_rows]);
        for i in 0..self_rows{
            result[[i]] = self[[i]] + other[[i]];
        }
        result
    }
}

impl<T: Add<Output = T> + AddAssign + Default + Copy> Add<Array<T, 2>> for Array<T, 2> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let (self_rows, self_cols) = (self.dims[0], self.dims[1]);
        let (other_rows, other_cols) = (other.dims[0], other.dims[1]);
        if self_rows != other_rows || self_cols != other_cols {
            panic!("matrix size does not match.");
        }
        let mut result = Self::zeros([self_rows, other_cols]);
        for i in 0..self_rows{
            for j in 0..other_cols{
                result[[i, j]] = self[[i, j]] + other[[i, j]];
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Array<T, 1>> for Array<T, 2> {
    type Output = Array<T, 1>;
    fn mul(self, other: Vector<T>) -> Self::Output {
        let (self_rows, self_cols) = (self.dims[0], self.dims[1]);
        let other_rows = other.dims[0];
        if self_cols != other_rows {
            panic!("matrix size does not match.");
            //return Err(MatrixError::UndefinedError("matrix size does not match.".to_string()));
        }
        let mut result = Vector::<T>::zeros([self_rows]);
        for i in 0..self_rows{
            let mut x = T::default();
            for k in 0..self_cols {
                x += self[[i, k]] * other[[k]];
            }
            result[[i]] = x;
        }
        result
    }
}

#[cfg(not(feature="blas"))]
impl<T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Array<T, 2>> for Array<T, 2> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let (self_rows, self_cols) = (self.dims[0], self.dims[1]);
        let (other_rows, other_cols) = (other.dims[0], other.dims[1]);
        if self_cols != other_rows {
            panic!("matrix size does not match.");
            //return Err(MatrixError::UndefinedError("matrix size does not match.".to_string()));
        }
        let mut result = Self::zeros([self_rows, other_cols]);
        for i in 0..self_rows{
            for j in 0..other_cols{
                let mut x = T::default();
                for k in 0..other_rows{
                    x += self[[i, k]] * other[[k, j]]
                }
                result[[i, j]] = x;
            }
        }
        result
    }
}



/// 転置
impl<T: Default + Copy> Transpose for Array<T, 2> {
    type Output = Array<T, 2>;
    fn transpose(self) -> Self::Output {
        let mut result = Array::<T, 2>::zeros([self.dims[1], self.dims[0]]);
        for i in 0..self.dims[0] {
            for j in 0..self.dims[1] {
                result[[j, i]] = self[[i, j]];
            }
        }
        result
    }
}

//impl<T: Mul<Output = T> + AddAssign + Default + Copy, const D: usize> Mul<Array<T, D>> for Array<T, D> {
//    type Output = Self;
//    fn mul(self, other: Self) -> Self::Output {
//        if self.cols != other.rows {
//            panic!("matrix size does not match.");
//            //return Err(MatrixError::UndefinedError("matrix size does not match.".to_string()));
//        }
//        let mut result = Self::zeros(self.rows, other.cols);
//        for i in 0..self.rows{
//            for j in 0..other.cols{
//                let mut x = T::default();
//                for k in 0..other.rows{
//                    x += self[(i, k)] * other[(k, j)]
//                }
//                result[(i, j)] = x;
//            }
//        }
//        result
//    }
//}
