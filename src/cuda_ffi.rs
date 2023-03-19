use std::{ptr, ffi::{c_float, c_int, c_void}};
use cublas_sys::*;
use cuda_runtime_sys::*;


#[derive(Debug, Clone)]
pub struct CUDAError;

pub fn malloc<T>(size: usize) -> Result<*mut T, CUDAError> {
    let mut ptr: *mut T = ptr::null_mut();
    let cuda_error =
    unsafe { cudaMalloc(&mut ptr as *mut *mut T as *mut *mut c_void, size) };
    if cuda_error == cudaError::cudaSuccess {
        assert_ne!(ptr,
            ptr::null_mut(),
            "cudaMalloc is succeeded, but returned null pointer!");
        Ok(ptr)
    } else {
        Err(CUDAError)
    }
}

pub fn memcpy<T>(dst: *mut T, src: *const T, size: usize, kind: cudaMemcpyKind) -> Result<(), CUDAError> {
    let cuda_error = unsafe {
        cudaMemcpy(dst as *mut c_void, src as *mut c_void, size, kind)
    };
    if cuda_error == cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(CUDAError)
    }
}

pub fn free<T>(devptr: *mut T) -> Result<(), CUDAError> {
    let cuda_error = unsafe { cudaFree(devptr as *mut c_void) };
    if cuda_error == cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(CUDAError)
    }
}

pub fn memcpy_to_device<T>(dst: *mut T, src: *const T, size: usize) -> Result<(), CUDAError> {
    memcpy(dst, src, size, cudaMemcpyKind::cudaMemcpyHostToDevice)
}

pub fn memcpy_to_host<T>(dst: *mut T, src: *const T, size: usize) -> Result<(), CUDAError> {
    memcpy(dst, src, size, cudaMemcpyKind::cudaMemcpyDeviceToHost)
}

