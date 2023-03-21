use crate::cuda_ffi::{free, malloc, memcpy_to_device, memcpy_to_host, CUDAError};
use cublas_sys::*;
use std::{
    ffi::{c_float, c_int, c_void},
    ptr,
};
//use crate::matrix::{Matrix};
use crate::vmatrix::{*};
use crate::basic_trait::{One};
use std::mem::size_of;
use std::ops::{Add, Mul, AddAssign, Index, IndexMut};

#[derive(Debug)]
pub struct CuMatrix<const ROWS: usize, const COLS: usize> {
    pub data_ptr: *mut f32
}

impl<const ROWS: usize, const COLS: usize> Mul<CuMatrix<ROWS, COLS>> for CuMatrix<ROWS, COLS> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let handle = CublasHandle::new().unwrap();
        let mut mat = Matrix::<f32, ROWS, COLS>::new();
        let mut result = mat.gpu();

        let n = ROWS*COLS*size_of::<c_float>();

        let alpha: c_float= 1.0;
        let beta: c_float= 1.0;

        let status = 
        unsafe {
            cublasSgemm_v2(
                handle.handle,
                cublasOperation_t::CUBLAS_OP_N,
                cublasOperation_t::CUBLAS_OP_N,
                n as i32,
                n as i32,
                n as i32,
                &alpha,
                self.data_ptr,
                n as i32,
                other.data_ptr,
                n as i32,
                &beta,
                result.data_ptr,
                n as i32,
            );
        };
        handle.destroy();
        result
    }
}



impl<const ROWS: usize, const COLS: usize> GPU for Matrix<f32, ROWS, COLS> {
    type Output = CuMatrix<ROWS, COLS>;
    fn gpu(&self) -> Self::Output {
        let n = ROWS*COLS*size_of::<c_float>();
        let mut a_ptr: *mut f32 = malloc(n).unwrap();
        memcpy_to_device(a_ptr, self.data.as_ptr() as *const f32, n).unwrap();
        CuMatrix { data_ptr: a_ptr }
    }
}
