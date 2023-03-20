use crate::cuda_ffi::{malloc, memcpy_to_device, memcpy_to_host, CUDAError};
use cublas_sys::*;
use std::{
    ffi::{c_float, c_int, c_void},
    ptr,
};
use crate::matrix::{Matrix};
use crate::vmatrix::VMatrix;
use crate::basic_trait::{One};
use std::mem::size_of;
use std::ops::{Add, Mul, AddAssign, Index, IndexMut};

#[derive(Debug, Clone)]
pub enum CuVMatrixError {
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

pub fn sgemm<const ROWS: usize, const COLS: usize>(
    handle: CublasHandle,
    a: Matrix::<f32, ROWS, COLS>,
    b: Matrix::<f32, ROWS, COLS>,
) {
    
}

#[derive(Debug)]
struct CuMatrix<const ROWS: usize, const COLS: usize> {
    pub data_ptr: *mut f32
}

#[derive(Debug)]
struct CuVMatrix {
    rows: usize,
    cols: usize,
    pub data_ptr: *mut f32
}


//impl CuMatrix<const ROWS: usize, const COLS: usize> {
//    fn cpu(&self) {
//
//    }
//}

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

impl Mul<CuVMatrix> for CuVMatrix {
    type Output = Result<Self, CuVMatrixError>;

    fn mul(self, other: Self) -> Self::Output {
        if self.cols != other.rows {
            return Err(VMatrixError::UndefinedError("matrix size does not match.".to_string()));
        }

        let handle = CublasHandle::new().unwrap();
        let mut mat = CuVMatrix::<f32>::new(self.row, other.cols);
        let mut result = mat.gpu();

        let n = self.rows*size_of::<c_float>();
        let k = self.cols*size_of::<c_float>();
        let m = other.cols*size_of::<c_float>();

        let alpha: c_float= 1.0;
        let beta: c_float= 0.0;

        let status = 
        unsafe {
            cublasSgemm_v2(
                handle.handle,
                cublasOperation_t::CUBLAS_OP_N,
                cublasOperation_t::CUBLAS_OP_N,
                n as i32,
                k as i32,
                m as i32,
                &alpha,
                self.data_ptr,
                n as i32,
                other.data_ptr,
                k as i32,
                &beta,
                result.data_ptr,
                m as i32,
            );
        };

        free(self.data_ptr).unwrap();
        free(other.data_ptr).unwrap();
        handle.destroy();
        Ok(result)
    }
}

trait GPU<const ROWS: usize, const COLS: usize> {
    fn gpu(&self) -> CuMatrix< ROWS, COLS>;
}

trait GPU {
    fn gpu(&self) -> CuVMatrix;
}

impl<const ROWS: usize, const COLS: usize> GPU<ROWS, COLS> for Matrix<f32, ROWS, COLS> 
{
    fn gpu(&self) -> CuMatrix<ROWS, COLS>{
        let n = ROWS*COLS*size_of::<c_float>();
        let mut a_ptr: *mut f32 = malloc(n).unwrap();
        memcpy_to_device(a_ptr, self.data.as_ptr() as *const f32, n).unwrap();
        CuMatrix { data_ptr: a_ptr }
    }
}

impl GPU for VMatrix<f32> 
{
    fn gpu(&self) -> CuVMatrix{
        let n = self.rows*self.cols*size_of::<c_float>();
        let mut a_ptr: *mut f32 = malloc(n).unwrap();
        memcpy_to_device(a_ptr, self.data.as_ptr() as *const f32, n).unwrap();
        CuVMatrix { data_ptr: a_ptr }
    }
}


pub fn cumatrix_test() {
    println!("cumatrix_test");
    let m_a = Matrix::<f32, 2, 2>::one();
    let m_a = m_a * 4_f32;
    let m_b = Matrix::<f32, 2, 2>::one();
    let m_b = m_b * 2_f32;

    println!("matrix:: {}", m_a);
    let cm_a = m_a.gpu();
    let cm_b = m_b.gpu();
    println!("cumatrix:: {:?}", cm_a);
    let cm_c = cm_a * cm_b;


    let mut h_c = vec![0.0f32; 4];
    memcpy_to_host(h_c.as_mut_ptr(), cm_c.data_ptr, 2*2*4).unwrap();
    println!("result: {:?}", h_c);
}

