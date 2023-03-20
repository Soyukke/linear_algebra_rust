#![crate_name = "linear_algebra"]

pub mod complex;
pub mod vector;
pub mod matrix;
pub mod vmatrix;
pub mod basic_trait;
pub mod blas_ffi;

#[cfg(feature="cuda")]
pub mod cuda_ffi;

#[cfg(feature="cuda")]
pub mod cublas_ffi;

#[cfg(feature="cuda")]
pub mod gpu;

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::matrix::{*};
    use crate::vmatrix::{*};
    use crate::basic_trait::{One, Transpose};
    use rand::Rng;
    use rand::distributions::{Distribution, Standard};
    use crate::blas_ffi::*;

    // cargo test mul_comp_mat -- --nocapture
    #[test]
    fn mul_comp_mat() {
        let mut n = Matrix::<Complex<f64>, 2, 2>::new();
        n[0][0] = Complex::new(3.0, 2.0);
        let mut l = Matrix::<Complex<f64>, 2, 2>::new();
        l[0][0] = Complex::new(3.0, -2.0);
        println!("{:?}", n*l);
        assert_eq!(2 + 2, 4);
    }

    // cargo test identity_matrix -- --nocapture
    #[test]
    fn identity_matrix() {
        let n = Matrix::<f64, 3, 3>::one();
        println!("{}", n);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn zeros_matrix_test() {
        let n = Matrix::<i32, 3, 3>::default();
        println!("{}", n);
        assert_eq!(n[1][0], 0);
    }

    #[test]
    fn random_test() {
        let mut rng = rand::thread_rng();
        let n: Matrix<f32, 4, 4> = rng.gen();
        println!("{}", n);
        assert!(n[0][0] < 1.0);
        assert!(0.0 <= n[0][0]);
    }

    #[test]
    fn transpose_test() {
        let mut rng = rand::thread_rng();
        let n: Matrix<f32, 2, 3> = rng.gen();
        println!("before transpose: {}", n);
        let n = n.transpose();
        println!("after  transpose: {}", n);
    }

    #[test]
    fn vmatrix_test() {
        let n: VMatrix<f32> = VMatrix::new(3, 3, 3.0_f32);
        let n2: VMatrix<f32> = VMatrix::zeros(2, 2);
        let n3: VMatrix<f32> = VMatrix::ones(2, 2);
        let n4: VMatrix<f32> = VMatrix::rand(2, 2);
        let n5 = n4.clone() * n4.clone();
        let n6a: VMatrix<f32> = VMatrix::rand(3, 2);
        let n6 = n6a.clone() * n6a.clone();
        let n7 = n5.clone().unwrap().transpose();
        println!("vmat: {}", n);
        println!("vmat: {}", n2);
        println!("vmat: {}", n3);
        println!("vmat: {}", n4);
        println!("vmat: {}", n5.unwrap());
        println!("vmat: {:?}", n6);
        println!("vmat: {}", n7);
    }

    #[test]
    fn vmatrix_test2() {
        let n: VMatrix<Complex<f32>> = VMatrix::new(3, 3, Complex{real: 1.0, imag: 2.0});
        println!("vmat: {:?}", n);
    }

    #[test]
    fn blas_test() {
        sgemm();
    }



}
