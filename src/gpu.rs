//use cuda_sys::*;
use cublas_sys::*;
use cuda_runtime_sys::*;
use std::{ptr, ffi::{c_float, c_int, c_void}};
use std::mem::size_of;
use std::io;
use crate::cuda_ffi::*;
use crate::cublas_ffi::*;


pub unsafe fn hoge() {
    println!("hogehogehoge");
    //let con = cublas::API::create();
    // https://github.com/autumnai/rust-cublas/blob/master/cublas/src/api/util.rs

    let mut handle: cublasHandle_t = ptr::null_mut();
    let res = match cublasCreate_v2(&mut handle) {
        cublasStatus_t::CUBLAS_STATUS_SUCCESS => Ok(handle),
        cublasStatus_t::CUBLAS_STATUS_NOT_INITIALIZED => {
            println!("CUBLAS_STATUS_NOT_INITIALIZED");
            Err(())
        },
        cublasStatus_t::CUBLAS_STATUS_ARCH_MISMATCH => {
            println!("CUBLAS_STATUS_ARCH_MISMATCH");
            Err(())
        },
        cublasStatus_t::CUBLAS_STATUS_ALLOC_FAILED => {
            println!("CUBLAS_STATUS_ALLOC_FAILED");
            Err(())},
        _ => {
            println!("not matched: ");
            Err(())
        }
    };
    println!("res::{:?}", res);



    const n: c_int = 2;
    let alpha: c_float= 1.0;
    let beta: c_float= 1.0;
    let mut a_mat: [c_float; 4] = [1.0, 2.0, 3.0, 4.0];
    //let mut a_ptr = a_mat.as_mut_ptr();
    let mut h_a_mat: [c_float; 4] = [1.0, 2.0, 3.0, 4.0];
    
        //cudaMemcpy(a_mut, a_mut, n * size_of(float), cudaMemcpyKind::cudaMemcpyHostToDevice);
    //
    //
    let nnn: usize = 4;
    let nnn_mem = nnn*size_of::<c_float>();
    let a_ptr: *mut f32 = malloc(nnn_mem).unwrap();
    let b_ptr: *mut f32 = malloc(nnn_mem).unwrap();
    let c_ptr: *mut f32 = malloc(nnn_mem).unwrap();

    let h_a = vec![3.0f32; nnn];
    let h_b = vec![2.0f32; nnn];
    let mut h_c = vec![0.0f32; nnn];

    memcpy_to_device(a_ptr, h_a.as_ptr(), nnn_mem).unwrap();
    memcpy_to_device(b_ptr, h_b.as_ptr(), nnn_mem).unwrap();

    let status = cublasSgemm_v2(
        handle,
        cublasOperation_t::CUBLAS_OP_N,
        cublasOperation_t::CUBLAS_OP_N,
        nnn as i32,
        nnn as i32,
        nnn as i32,
        &alpha,
        a_ptr,
        nnn as i32,
        b_ptr,
        nnn as i32,
        &beta,
        c_ptr,
        nnn as i32,
    );

    memcpy_to_host(h_c.as_mut_ptr(), c_ptr, nnn_mem).unwrap();

    free(a_ptr).unwrap();
    free(b_ptr).unwrap();
    free(c_ptr).unwrap();

    let destroy_status = cublasDestroy_v2(handle);

    println!("result status::{:?}", status);
    println!("result c_mat::{:?}", h_c);
    println!("result destroy status::{:?}", destroy_status);


}
