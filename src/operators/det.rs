use crate::{Matrix, Vector};
use crate::basic_trait::One;
use std::ops::{AddAssign,Mul,Add, Sub, SubAssign, Div};

impl<T: Mul<Output = T> + Sub<Output=T> + Div<Output=T> + AddAssign + Default + Copy + One + SubAssign> Matrix<T> {
    fn det(&self) -> Matrix<T> {
        todo!();
    }

    
    fn inv(&self) -> Matrix<T> {
        todo!();
    }

    pub fn lu(&self) -> (Matrix<T>, Matrix<T>) {
        let (n, m) = (self.dims[0], self.dims[1]);
        let mut l = Matrix::<T>::identity([n, n]);
        let mut u = self.clone();

        // LU分解を実行
        for k in 0..n {
            for i in k+1..n {
                //  k < iを満たす。下三角領域
                let coef = u[[i, k]] / u[[k, k]];
                l[[i, k]] = coef;
                for j in k..n {
                    // 上三角部分
                    u[[i, j]] = u[[i, j]] - coef * u[[k, j]];
                }
            }
        }

        // LU分解を実行
        //for k in 0..n {
        //    for i in k..n {
        //        let mut sum = T::default();
        //        for p in 0..k {
        //            sum += l[[i, p]] * u[[p, k]];
        //        }
        //        u[[i, k]] -= sum;
        //    }
        //    for j in k+1..n {
        //        let mut sum = T::default();
        //        for p in 0..k {
        //            sum += l[[k, p]] * u[[p, j]];
        //        }
        //        l[[k, j]] = (self[[k, j]] - sum) / u[[k, k]];
        //    }
        //}
        (l, u)
    }

    fn svd(&self) -> Matrix<T> {
        todo!();
    }
}
