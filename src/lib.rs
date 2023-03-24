#![crate_name = "linear_algebra"]

pub mod complex;
pub mod vector;
pub mod basic_trait;
pub mod basic_traits;

pub use basic_traits::*;

pub mod array;
#[cfg(feature="blas")]
pub mod array_blas;
#[cfg(feature="blas")]
pub mod lapack_ffi;


pub use crate::array::*;
pub use crate::complex::*;

pub mod solver;
pub use crate::solver::*;

#[cfg(feature="blas")]
pub use crate::array_blas::*;
#[cfg(feature="blas")]
pub use crate::lapack_ffi::*;

#[cfg(feature="cuda")]
pub mod cuda_ffi;

#[cfg(feature="cuda")]
pub mod cublas_ffi;
#[cfg(feature="cuda")]
pub use crate::cublas_ffi::*;

#[cfg(feature="cuda")]
pub mod cusolver_ffi;

#[cfg(feature="cuda")]
pub mod gpu;

#[cfg(test)]
mod tests {

    use crate::complex::Complex;
    use crate::basic_trait::{One, Transpose};
    use rand::Rng;
    use rand::distributions::{Distribution, Standard};


    #[cfg(feature="blas")]
    #[test]
    fn blas_test() {
        use crate::blas_ffi::*;
        sgemm();
    }

    #[cfg(feature="cuda")]
    #[test]
    fn cublas_sgemm() {
        use crate::vmatrix::{*};
        use crate::cublas_ffi::*;
        cublasversion();
        let x = Matrix::<f32>::new(3, 3, 2f32);
        let y = Matrix::<f32>::new(3, 3, 3f32);
        println!("cpu::x: {}", x);
        println!("cpu::y: {}", y);
        let cx = x.gpu();
        let cy = y.gpu();
        let cz = cx * cy;
        let cpuz = cz.unwrap().cpu();
        println!("cpu::z: {}", cpuz);
    }

    #[cfg(feature="cuda")]
    #[test]
    fn cusolver_01() {
        use crate::vmatrix::{*};
        use crate::cusolver_ffi::*;
        cusolverDnZheevd_ffi();
    }

    #[cfg(feature="cuda")]
    #[test]
    fn cusolver_02() {
        use crate::vmatrix::{*};
        use crate::cusolver_ffi::*;
        cusolverDnSgeqrf_ffi();
    }


    #[cfg(feature="cuda")]
    #[test]
    fn cusolver_00() {
        use crate::cusolver_ffi::*;
        cusolverGetVersion_ffi();
    }

    #[cfg(feature="cuda")]
    #[test]
    fn cusolver_99() {
        use crate::cusolver_ffi::*;
        sgemm_test();
    }
}
