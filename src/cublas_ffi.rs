use crate::cuda_ffi::{free, malloc, memcpy_to_device, memcpy_to_host, CUDAError};
use cublas_sys::*;
use std::{
    ffi::{c_float, c_int, c_void},
    ptr,
};
//use crate::matrix::{Matrix};
use crate::Matrix;
use crate::basic_trait::One;
use std::mem::size_of;
use std::ops::{Add, Mul, AddAssign, Index, IndexMut};

#[derive(Debug, Clone)]
pub enum CuMatrixError {
    UndefinedError(String),
}

pub struct CublasHandle {
    pub handle: cublasHandle_t,
}

impl CublasHandle {
    fn new() -> Result<Self, CUDAError> {
        let mut handle: cublasHandle_t = ptr::null_mut();
        let result = unsafe {
            match cublasCreate_v2(&mut handle) {
                cublasStatus_t::CUBLAS_STATUS_SUCCESS => Ok(Self { handle }),
                cublasStatus_t::CUBLAS_STATUS_NOT_INITIALIZED => {
                    println!("CUBLAS_STATUS_NOT_INITIALIZED");
                    Err(CUDAError)
                }
                cublasStatus_t::CUBLAS_STATUS_ARCH_MISMATCH => {
                    println!("CUBLAS_STATUS_ARCH_MISMATCH");
                    Err(CUDAError)
                }
                cublasStatus_t::CUBLAS_STATUS_ALLOC_FAILED => {
                    println!("CUBLAS_STATUS_ALLOC_FAILED");
                    Err(CUDAError)
                }
                _ => {
                    println!("not matched: ");
                    Err(CUDAError)
                }
            }
        };
        result
    }

    fn destroy(&self) {
        let destroy_status = unsafe {
            cublasDestroy_v2(self.handle);
        };
        println!("CublasHandle::destroy::{:?}", destroy_status);
    }
}

pub fn cublasversion() {
    unsafe {
        let handle = CublasHandle::new().unwrap();
        let mut result = 0;
        cublasGetVersion_v2(handle.handle, &mut result);
        println!("CUBLAS Version : {}", result);
    }
}

#[derive(Debug)]
pub struct CuMatrix {
    rows: usize,
    cols: usize,
    pub data_ptr: *mut f32
}

impl Mul<CuMatrix> for CuMatrix {
    type Output = Result<Self, CuMatrixError>;

    fn mul(self, other: Self) -> Self::Output {
        if self.cols != other.rows {
            return Err(CuMatrixError::UndefinedError("matrix size does not match.".to_string()));
        }

        let handle = CublasHandle::new().unwrap();
        let mut mat = Matrix::<f32>::zeros([self.rows, other.cols]);
        let mut result = mat.gpu();

        let m = self.rows;
        let n = other.cols;
        let k = self.cols; // = other.rows


        let alpha: c_float= 1.0;
        let beta: c_float= 0.0;

        let status = 
        unsafe {
            cublasSgemm_v2(
                handle.handle,
                cublasOperation_t::CUBLAS_OP_N,
                cublasOperation_t::CUBLAS_OP_N,
                m as i32,
                n as i32,
                k as i32,
                &alpha,
                self.data_ptr,
                m as i32,
                other.data_ptr,
                n as i32,
                &beta,
                result.data_ptr,
                k as i32,
            );
        };

        free(self.data_ptr).unwrap();
        free(other.data_ptr).unwrap();
        handle.destroy();
        Ok(result)
    }
}

pub trait GPU {
    type Output;
    fn gpu(&self) -> Self::Output;
}

impl GPU for Matrix<f32> {
    type Output = CuMatrix;
    fn gpu(&self) -> Self::Output {
        let (rows, cols) = (self.dims[0], self.dims[1]);
        let n = rows*cols*size_of::<c_float>();
        let mut a_ptr: *mut f32 = malloc(n).unwrap();
        memcpy_to_device(a_ptr, self.data.as_ptr(), n).unwrap();
        CuMatrix { rows: rows, cols: cols, data_ptr: a_ptr }
    }
}

pub trait CPU {
    type Output;
    fn cpu(&self) -> Self::Output;
}

impl CPU for CuMatrix {
    type Output = Matrix<f32>;
    fn cpu(&self) -> Self::Output {
        let mut data = vec![0.0f32; self.rows*self.cols];
        let n = self.rows*self.cols*size_of::<c_float>();
        memcpy_to_host(data.as_mut_ptr(), self.data_ptr, n).unwrap();
        Matrix { dims: [self.rows, self.cols], data: data}
    }
}

