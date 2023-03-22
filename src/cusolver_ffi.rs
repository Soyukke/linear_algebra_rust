use crate::complex::*;
use crate::cuda_ffi::{free, malloc, memcpy_to_device, memcpy_to_host, CUDAError};
use crate::vmatrix::*;
use cusolver_sys::*;
use std::{
    ffi::{c_float, c_void},
    ptr,
};
use std::os::raw::c_int;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub enum CusolverError {
    UndefinedError(String),
}

pub fn cusolverDnCheevd_ffi() {
    println!("cusolverDnZheevd");
    //cusolverDnZheevd(
    //    handle: cusolverDnHandle_t,
    //    jobz: cusolverEigMode_t,
    //    uplo: cublasFillMode_t,
    //    n: ::std::os::raw::c_int,
    //    A: *mut cuDoubleComplex,
    //    lda: ::std::os::raw::c_int,
    //    W: *mut f64,
    //    work: *mut cuDoubleComplex,
    //    lwork: ::std::os::raw::c_int,
    //    info: *mut ::std::os::raw::c_int,
    //);
    //
    //
    let n = 3;
    let mut A = Matrix::<Complex<f32>>::rand(n, n);
    let mut W = vec![0f32; n as usize];
    let mut work = Matrix::<Complex<f32>>::zeros(n * n, 1);
    let mut lwork = (n * n) as c_int;
    let mut info = vec![0 as c_int; 1];
    println!("A: {}", A);
    let mut a_vec = A.to_vec();
    let mut work_vec = work.to_vec();

    let n_mem1 = n * n * size_of::<float2>();
    let n_mem2 = n * size_of::<c_float>();
    let n_mem3 = n * n * size_of::<float2>();
    println!("n_mem3: {}", n_mem3);
    let mut a_ptr1: *mut float2 = malloc(n_mem1).unwrap();
    let mut a_ptr2: *mut f32 = malloc(n_mem2).unwrap();
    let mut a_ptr3: *mut float2 = malloc(n_mem3).unwrap();
    println!("a_ptr1: {:?}", a_ptr1);
    println!("a_ptr2: {:?}", a_ptr2);
    println!("a_ptr3: {:?}", a_ptr3);
    let mut a_vec_test = vec![float2{x: 0.0f32, y: 0.0f32}; n*n];
    a_vec_test[1] = float2{x: 3.0f32, y: 0.0f32};
    a_vec_test[3] = float2{x: 5.0f32, y: 0.0f32};
    a_vec_test[7] = float2{x: 8.0f32, y: 0.0f32};
    memcpy_to_device(a_ptr1, a_vec_test.as_ptr(), n_mem1).unwrap();
    //memcpy_to_device(a_ptr2, W.as_ptr(), n_mem2).unwrap();
    //memcpy_to_device(a_ptr3, work_vec.as_ptr() as *const float2, n_mem3).unwrap();
    let mut resA = vec![float2{x: 0.0,y: 0.0}; n*n];
    let resA_s = memcpy_to_host(resA.as_mut_ptr(), a_ptr1, n_mem1);
    println!("resA: {:?}", resA_s);
    println!("resA data:: {:?}", resA);


    unsafe {
        let mut handle: cusolverDnHandle_t = ptr::null_mut();
        let status_1 = match cusolverDnCreate(&mut handle) {
            cusolverStatus_t_CUSOLVER_STATUS_SUCCESS => Ok(()),
            _ => Err("failed: undefined or not implemented"),
        };
        println!("status1: {:?}", status_1);

        let status_bf = cusolverDnCheevd_bufferSize(
            handle,
            cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
            cublasDiagType_t_CUBLAS_DIAG_NON_UNIT,
            n as c_int,
            a_ptr1,
            n as c_int,
            a_ptr2,
            &mut lwork,
        );
        println!("status_bf::{:?}, lwork::{:?}", status_bf, lwork);
        //memcpy_to_device(a_ptr3, work_vec.as_ptr() as *const float2, lwork).unwrap();
        //cudaStat1 = cudaMalloc((void**)&d_work, lwork*sizeof(double));
        let mut a_ptr3: *mut float2 = malloc((lwork as usize)*size_of::<float2>()).unwrap();
        println!("a_ptr3::{:?}", a_ptr3);

        
        cusolverDnCheevd(
            handle,
            cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
            cublasFillMode_t_CUBLAS_FILL_MODE_LOWER,
            n as c_int,
            a_ptr1,
            n as c_int,
            a_ptr2,
            a_ptr3,
            lwork,
            info.as_mut_ptr(),
        );
        println!("a_ptr3: {:?}", a_ptr3);

        //let mut res3 = vec![float2 { x: 0f32, y: 0f32 }; (n * n) as usize];
        //let res3_1 = memcpy_to_host(res3.as_mut_ptr(), a_ptr3, n_mem3);
        //println!("res3_1_2: {:?}", res3_1);
        //

        //let mut resA = vec![float2{x: 0.0,y: 0.0}; n*n];
        //let resA_s = memcpy_to_host(resA.as_mut_ptr(), a_ptr1, n_mem1);
        //println!("resA: {:?}", resA_s);
        //println!("resA data:: {:?}", resA);

        //let mut res2 = vec![0f32; n];
        //let res011 = memcpy_to_host(res2.as_mut_ptr(), a_ptr2, n_mem2);
        //println!("res011: {:?}", res011);

        //let mut res3 = vec![float2 { x: 0f32, y: 0f32 }; lwork as usize];
        //let calced_mem_3 = (lwork as usize)*size_of::<float2>();
        //let res3_1 = memcpy_to_host(res3.as_mut_ptr(), a_ptr3, calced_mem_3);
        //println!("res3_1: {:?}", res3_1);

        println!("info: {:?}", info);
        println!("work: {:?}", work);
        println!("work_vec: {:?}", work_vec);
        println!("a_vec: {:?}", a_vec);

        free(a_ptr1);
        free(a_ptr2);
        free(a_ptr3);
        let status_2 = cusolverDnDestroy(handle);
        println!("status2: {:?}", status_2);
    }
}

pub fn cusolverDnZheevd_ffi() {
    println!("cusolverDnZheevd");
    let n = 3;
    let mut W = vec![0f64; n as usize];
    let mut lwork = 0 as c_int;
    let mut info = vec![0 as c_int; 1];

    let A_size = n * n * size_of::<double2>();
    let W_size = n * size_of::<f64>();

    let mut A_ptr: *mut double2 = malloc(A_size).unwrap();
    let mut W_ptr: *mut f64 = malloc(W_size).unwrap();
    let mut a_vec_test = vec![double2{x: 0.0, y: 0.0}; n*n];
    a_vec_test[0] = double2{x: 13.0, y: 0.0};
    a_vec_test[1] = double2{x: 3.0, y: 0.0};
    a_vec_test[2] = double2{x: -7.0, y: 0.0};
    a_vec_test[3] = double2{x: 5.0, y: 0.0};
    a_vec_test[7] = double2{x: 8.0, y: 0.0};
    a_vec_test[8] = double2{x: 100.0, y: 0.0};
    memcpy_to_device(A_ptr, a_vec_test.as_ptr(), A_size).unwrap();

    unsafe {
        let mut handle: cusolverDnHandle_t = ptr::null_mut();
        let status_1 = match cusolverDnCreate(&mut handle) {
            cusolverStatus_t_CUSOLVER_STATUS_SUCCESS => Ok(()),
            _ => Err("failed: undefined or not implemented"),
        };
        println!("status1: {:?}", status_1);

        let status_bf = cusolverDnZheevd_bufferSize(
            handle,
            cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
            cublasDiagType_t_CUBLAS_DIAG_NON_UNIT,
            n as c_int,
            A_ptr,
            n as c_int,
            W_ptr,
            &mut lwork,
        );
        println!("status_bf::{:?}, lwork::{:?}", status_bf, lwork);

        let work_mem_size = (lwork as usize)*size_of::<double2>();
        let mut work_ptr: *mut double2 = malloc((lwork as usize)*size_of::<double2>()).unwrap();
        println!("work_ptr::{:?}", work_ptr);

        //let mut res2 = vec![0f64; n];
        //let res_W = memcpy_to_host(res2.as_mut_ptr(), W_ptr, W_size);
        //println!("result W: {:?}", res_W);

        let status_solver = cusolverDnZheevd(
            handle,
            cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
            cublasFillMode_t_CUBLAS_FILL_MODE_LOWER,
            n as c_int,
            A_ptr,
            n as c_int,
            W_ptr,
            work_ptr,
            lwork,
            info.as_mut_ptr(),
        );
        println!("status solver: {:?}", status_solver);

        let mut res2 = vec![0f64; n];
        let res_W = memcpy_to_host(res2.as_mut_ptr(), W_ptr, W_size);
        println!("result W: {:?}", res_W);

        println!("info: {:?}", info);

        free(A_ptr);
        free(W_ptr);
        free(work_ptr);
        let status_2 = cusolverDnDestroy(handle);
        println!("status2: {:?}", status_2);
    }
}

pub fn cusolverDnSgeqrf_ffi() {
    println!("cusolverDnSgeqrf_ffi");
    let n = 3;
    let mut TAU = vec![0f32; n as usize];
    let mut lwork = 0 as c_int;
    let mut info = vec![0 as c_int; 1];

    let A_size = n * n * size_of::<f32>();
    let TAU_size = n * size_of::<f32>();

    let mut A_ptr: *mut f32 = malloc(A_size).unwrap();
    let mut TAU_ptr: *mut f32 = malloc(TAU_size).unwrap();
    let mut a_vec_test = vec![0f32; n*n];
    a_vec_test[0] = 13.0;
    a_vec_test[1] = 3.0;
    a_vec_test[2] = -7.0;
    a_vec_test[3] = 5.0;
    a_vec_test[7] = 8.0;
    a_vec_test[8] = 100.0;
    memcpy_to_device(A_ptr, a_vec_test.as_ptr(), A_size).unwrap();

    unsafe {
        let mut handle: cusolverDnHandle_t = ptr::null_mut();
        let status_1 = match cusolverDnCreate(&mut handle) {
            cusolverStatus_t_CUSOLVER_STATUS_SUCCESS => Ok(()),
            _ => Err("failed: undefined or not implemented"),
        };
        println!("status1: {:?}", status_1);

        let status_bf = cusolverDnSgeqrf_bufferSize(
            handle,
            n as c_int,
            n as c_int,
            A_ptr,
            n as c_int,
            &mut lwork,
        );
        println!("status_bf::{:?}, lwork::{:?}", status_bf, lwork);

        let work_mem_size = (lwork as usize)*size_of::<f32>();
        let mut work_ptr: *mut f32 = malloc((lwork as usize)*size_of::<f32>()).unwrap();
        println!("work_ptr::{:?}", work_ptr);

        //let mut res2 = vec![0f64; n];
        //let res_W = memcpy_to_host(res2.as_mut_ptr(), W_ptr, W_size);
        //println!("result W: {:?}", res_W);

        let status_solver = cusolverDnSgeqrf(
            handle,
            n as c_int,
            n as c_int,
            A_ptr,
            n as c_int,
            TAU_ptr,
            work_ptr,
            lwork,
            info.as_mut_ptr(),
        );
        println!("status solver: {:?}", status_solver);

        let mut res2 = vec![0f32; n];
        let res_TAU = memcpy_to_host(res2.as_mut_ptr(), TAU_ptr, TAU_size);
        println!("result TAU: {:?}", res_TAU);

        println!("info: {:?}", info);

        free(A_ptr);
        free(TAU_ptr);
        free(work_ptr);
        let status_2 = cusolverDnDestroy(handle);
        println!("status2: {:?}", status_2);
    }
}
