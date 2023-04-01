use std::{ops::{MulAssign, AddAssign, Mul, Range, Sub, SubAssign}, iter::Chain};
use num_traits::real::Real;

use crate::{Matrix, Vector, ArrayValue, Transpose, One};

#[derive(Debug, Clone)]
pub struct MatrixView<T, S>
where S: Iterator<Item=usize>
    {
    matrix: Matrix<T>,
    rows: S,
    cols: S,
}

impl<T: Default + Copy, S: Iterator<Item=usize>+ Clone> MatrixView<T, S> {

    pub fn new(arr: Matrix<T>, rows: S, cols: S) -> Self {
        MatrixView { matrix: arr, rows: rows, cols: cols }
    }

    pub fn array(self) -> Matrix<T> {
        self.matrix
    }

    pub fn subarray(&self) -> Matrix<T> {
        let mut m = Matrix::<T>::zeros([self.rows.clone().count(), self.cols.clone().count()]);
        for (i, s) in self.rows.clone().enumerate() {
            for (j, t) in self.cols.clone().enumerate() {
                m[[i, j]] = self.matrix[[s, t]];
            }
        }
        m
    }

    pub fn set_subarray(mut self, v: Matrix<T>) -> Matrix<T> {
        for (i, row) in self.rows.clone().enumerate() {
            for (j, col) in self.cols.clone().enumerate() {
                self.matrix[[row, col]] = v[[i, j]];
            }
        }
        self.matrix
    }
}

impl<T: Default + Copy + Clone> Matrix<T> {
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

    //pub fn slice_mut(&mut self, rows: Range<usize>, cols: Range<usize>) -> &mut [T] {
    pub fn view(self, rows: Range<usize>, cols: Range<usize>) -> MatrixView<T, Range<usize>>
    {
        let r = rows;
        let c = cols;
        MatrixView::new(self, r, c)
    }

    pub fn view2<S: Iterator<Item=usize> + Clone>(self, rows: S, cols: S) -> MatrixView<T, S>
    {
        let r = rows;
        let c = cols;
        MatrixView::new(self, r, c)
    }


    pub fn submatrix(self, i: usize, j: usize) -> MatrixView<T, Chain<Range<usize>, Range<usize>>> {
        let (n, m) = (self.dims[0], self.dims[1]);
        let r = (0..i).chain(i+1..n);
        let c = (0..j).chain(j+1..m);
        MatrixView::new(self, r, c)
    }

    pub fn subarray<A: Iterator<Item=usize> + ExactSizeIterator + Copy>(&self, rows: A, cols: A) -> Matrix<T> {
        let mut m = Matrix::<T>::zeros([rows.len(), cols.len()]);
        for (i, s) in rows.enumerate() {
            for (j, t) in cols.enumerate() {
                m[[i, j]] = self[[s, t]];
            }
        }
        m
    }

    pub fn set_subarray<A: Iterator<Item=usize> + ExactSizeIterator + Copy>(&self, rows: A, cols: A) -> Matrix<T> {
        let mut m = Matrix::<T>::zeros([rows.len(), cols.len()]);
        for (i, s) in rows.enumerate() {
            for (j, t) in cols.enumerate() {
                m[[i, j]] = self[[s, t]];
            }
        }
        m
    }

    pub fn upper_triangular(m: &Matrix<T>) -> Matrix<T> {
        let mut u = Matrix::<T>::zeros(m.dims);
        for i in 0..m.dims[0] {
            for j in i..m.dims[1] {
                u[[i, j]] = m[[i, j]];
            }
        }
        u
    }

    
    


}

impl<T: Default + Copy + Clone + One> Matrix<T> {

    pub fn lower_triangular(m: &Matrix<T>) -> Matrix<T> {
        let mut l = Matrix::<T>::identity(m.dims);
        for j in 0..m.dims[0] {
            for i in j+1..m.dims[1] {
                l[[i, j]] = m[[i, j]];
            }
        }
        l
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

impl<T: Real + MulAssign + AddAssign + Default + Clone + One + Sub<Output=T> + Mul<Output=T> + std::fmt::Display + SubAssign> Matrix<T> {

    /// A = QR
    /// R: 上三角行列
    /// Q
    #[cfg(not(feature="blas"))]
    pub fn qr(&self) -> (Matrix<T>, Matrix<T>) {
        let (n, m) = (self.dims[0], self.dims[1]);
        //let mut q = Matrix::<T> {dims: [n, n], data: vec![T::default(); n * n]};
        let mut q = Matrix::<T>::identity([n, n]);
        let mut r = self.clone();

        for k in 0..n.min(m) {
            let x = r.clone().view(k..n, k..k+1).subarray();
            let sub_a = r.clone().view(k..n, k..m).subarray();
            let mut v = x.clone();
            let alpha = v[[0, 0]].signum() * x.l2_norm();
            let e1 = Matrix::<T>::identity([n-k, 1]);
            v = x + e1 * alpha;
            v = v.clone() / v.l2_norm();

            let newr = &sub_a - (&v * (v.transpose() * &sub_a) * T::from(2.0).unwrap());
            r = r.view(k..n, k..m).set_subarray(newr);
            let qsub = q.clone().view(0..n, k..n).subarray();
            let newq = &qsub - (&qsub * &v) * v.transpose() * T::from(2.0).unwrap();
            q = q.clone().view(0..n, k..n).set_subarray(newq);
        }
        (q, r)
    }

}
