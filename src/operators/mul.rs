use crate::{Matrix, Vector};
use std::ops::{AddAssign,Mul,Add};

impl<T: Mul<Output = T> + AddAssign + Default + Copy> Matrix<T> {
    fn _mul(&self, other: &Self) -> Matrix<T> {
        let (self_rows, self_cols) = (self.dims[0], self.dims[1]);
        let (other_rows, other_cols) = (other.dims[0], other.dims[1]);
        if self_cols != other_rows {
            panic!("matrix size does not match.");
            //return Err(MatrixError::UndefinedError("matrix size does not match.".to_string()));
        }
        let mut result = Matrix::<T>::zeros([self_rows, other_cols]);
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

    fn _mul_vec(&self, other: &Vector<T>) -> Vector<T> {
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

impl<'a, 'b, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: &'b Matrix<T>) -> Self::Output {
        self._mul(other)
    }
}

impl<'b, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: &'b Matrix<T>) -> Self::Output {
        self._mul(other)
    }
}

impl<'a, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        self._mul(&other)
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        self._mul(&other)
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, other: Vector<T>) -> Self::Output {
        self._mul_vec(&other)
    }
}

impl<'a, 'b, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<&'b Vector<T>> for &'a Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, other: &'b Vector<T>) -> Self::Output {
        self._mul_vec(other)
    }
}

impl<'a, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<Vector<T>> for &'a Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, other: Vector<T>) -> Self::Output {
        self._mul_vec(&other)
    }
}

impl<'b, T: Mul<Output = T> + AddAssign + Default + Copy> Mul<&'b Vector<T>> for Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, other: &'b Vector<T>) -> Self::Output {
        self._mul_vec(other)
    }
}
