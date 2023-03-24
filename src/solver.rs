use std::ops::{MulAssign, AddAssign, Mul};
use num_traits::real::Real;

use crate::{Matrix, Vector, Transpose};
use crate::basic_trait::One;

impl<T: Clone> Matrix<T> {
    pub fn slice(&self, rstart: usize, rend: usize, cstart: usize, cend: usize) -> Matrix<T> {
        let nrows = rend - rstart;
        let ncols = cend - cstart;
        let mut data: Vec<T> = Vec::with_capacity(nrows * ncols);

        for i in rstart..rend {
            let row_start = i * self.dims[1] + cstart;
            let row_end = row_start + ncols;
            data.extend_from_slice(&self.data[row_start..row_end]);
        }

        Matrix {
            dims: [nrows, ncols],
            data,
        }
    }
}


impl<T: Default + Copy + MulAssign + AddAssign + PartialOrd + Mul<Output=T> + Real> Vector<T> {
    pub fn l1_norm(&self) -> T {
        let mut sum = T::default();
        for i in 0..self.dims[0] {
            sum += self.data[i];
        }
        sum
    }

    pub fn l2_norm(&self) -> T {
        let mut sum = T::default();
        for i in 0..self.dims[0] {
            sum += self.data[i] * self.data[i];
        }
        sum.sqrt()
    }
}

impl<T: Default + Copy + MulAssign + AddAssign + PartialOrd + Real> Matrix<T> {
    pub fn l1_norm(&self) -> T {
        let mut sum = T::default();
        for j in 0..self.dims[1] {
            let mut col_sum = T::default();
            for i in 0..self.dims[0] {
                col_sum += self.data[i * self.dims[1] + j];
            }
            if col_sum > sum {
                sum = col_sum;
            }
        }
        sum
    }

    pub fn l2_norm(&self) -> T {
        let mut sum = T::default();
        for j in 0..self.dims[1] {
            let mut col_sum = T::default();
            for i in 0..self.dims[0] {
                col_sum += self.data[i * self.dims[1] + j] * self.data[i * self.dims[1] + j];
            }
            sum += col_sum.sqrt();
        }
        sum
    }

    pub fn fro_norm(&self) -> T {
        let mut sum = T::default();
        for i in 0..self.data.len() {
            sum += self.data[i] * self.data[i];
        }
        sum.sqrt()
    }
}

//impl<T: Default + Copy + One + Real + MulAssign + AddAssign> Matrix<T> {
//    pub fn qr_decomposition(&self) -> (Matrix<T>, Matrix<T>) {
//        let (m, n) = (self.dims[0], self.dims[1]);
//        let mut q = Matrix::<T>::identity([m, m]);
//        let mut r = self.clone();
//
//        for j in 0..n {
//            let x = r.slice(j, j, m-j, 1);
//            let norm_x = x.l2_norm();
//
//            if norm_x.abs() < T::from(f32::EPSILON).unwrap() {
//                continue;
//            }
//
//            let mut u = x.clone();
//            u[[0, 0]] += if x[[0, 0]] > T::default() { norm_x } else { -norm_x };
//            let beta = T::from(2.0).unwrap() / (u.clone().transpose() * u.clone())[[0, 0]];
//            let p = Matrix::<T>::identity([m-j, m-j]) - (u.clone() * u.clone()) * beta;
//            // TODO:
//            //let p = Matrix::<T>::identity([m-j, m-j]) * beta;
//
//            let q_sub = Matrix::<T>::identity([m, m]);
//            // TODO: 
//            //q_sub.slice_mut((j, j), (m-j, m-j)).set_subarray(&p);
//            q = q * q_sub.clone();
//            r = q_sub * r;
//        }
//
//        (q, r)
//    }
//}
