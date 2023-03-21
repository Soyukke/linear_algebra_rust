use cusolver_sys::*;
use std::{ptr, ffi::{c_float, c_int, c_void}};

pub fn cusolverDnZheevd() {
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
    unsafe {
        let mut handle: cusolverDnHandle_t = ptr::null_mut();
        let status = cusolverDnCreate(&mut handle);
        println!("status1: {:?}", status);
        let status = cusolverDnDestroy(handle);
        println!("status2: {:?}", status);
        //cusolverDnZheevd(
        //    handle: cusolverDnHandle_t,
        //    jobz: cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR,
        //    uplo: cublasFillMode_t,
        //    n: ::std::os::raw::c_int,
        //    A: *mut cuDoubleComplex,
        //    lda: ::std::os::raw::c_int,
        //    W: *mut f64,
        //    work: *mut cuDoubleComplex,
        //    lwork: ::std::os::raw::c_int,
        //    info: *mut ::std::os::raw::c_int,
        //);

    }

}
