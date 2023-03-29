use crate::{Matrix, Vector};

impl<T: Mul<Output = T> + AddAssign + Default + Copy> Matrix<T> {
    fn det(&self) -> Matrix<T> {
    }

    
    fn inv(&self) -> Matrix<T> {
    }

    fn lu(&self) -> (Matrix<T>, Matrix<T>) {
    }

    fn svd(&self) -> Matrix<T> {

    }
}
