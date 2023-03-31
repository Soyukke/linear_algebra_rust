use crate::array::{Array, Vector, Matrix};
use crate::complex::Complex;
use crate::basic_trait::One;

use blas_sys::*;
use lapack_src::*;
use lapack_sys::*;
use openblas_src::*;
use std::{
    ffi::{c_char, c_float, c_int, c_void, CString},
    ptr,
};

use std::cmp::min;

pub struct EigenResult<T, S> {
    pub values: Vector<T>,
    pub vectors_l: Matrix<S>,
    pub vectors_r: Matrix<S>,
}

pub trait Eigen {
    type Output;
    fn eigen(&mut self) -> Self::Output;
}

impl Eigen for Array<f64, 2> {
    type Output = EigenResult<Complex<f64>, f64>;
    fn eigen(&mut self) -> Self::Output {
        let jobvl: *mut c_char = CString::new("V").unwrap().into_raw();
        let jobvr: *mut c_char = CString::new("V").unwrap().into_raw();
        let n: i32 = self.dims[0] as i32;
        let u = self.dims[0];
        let lda: i32 = n;
        let ldvl: i32 = n;
        let ldvr: i32 = n;
        let mut wr = Vector::<f64>::zeros([n as usize]);
        let mut wi = Vector::<f64>::zeros([n as usize]);
        let mut vl = Matrix::<f64>::zeros([n as usize, n as usize]);
        let mut vr = Matrix::<f64>::zeros([n as usize, n as usize]);
        let mut work = Vector::<f64>::zeros([1] as [usize; 1]);
        let mut lwork: i32 = -1;
        let mut info: i32 = 0;
        // 実非対称固有値問題
        unsafe {
            // https://docs.rs/lapack-sys/latest/lapack_sys/fn.dgeev_.html
            dgeev_(
                jobvl,
                jobvr,
                &n,
                self.data.as_mut_ptr(),
                &lda,
                wr.data.as_mut_ptr(),
                wi.data.as_mut_ptr(),
                vl.data.as_mut_ptr(),
                &ldvl,
                vr.data.as_mut_ptr(),
                &ldvr,
                work.data.as_mut_ptr(),
                &mut lwork,
                &mut info,
            );
            //println!("dgeev_::(lwork, info)::({}, {})", lwork, info);

            let mut lwork = work[[0]] as i32;
            let mut work = Vector::<f64>::zeros([lwork as usize]);
            dgeev_(
                jobvl,
                jobvr,
                &n,
                self.data.as_mut_ptr(),
                &lda,
                wr.data.as_mut_ptr(),
                wi.data.as_mut_ptr(),
                vl.data.as_mut_ptr(),
                &ldvl,
                vr.data.as_mut_ptr(),
                &ldvr,
                work.data.as_mut_ptr(),
                &mut lwork,
                &mut info,
            );

        }
        // 固有値
        let mut values: Vector<Complex<f64>> = Vector::zeros([u]);
        for i in 0..u {
            values[[i]] = Complex {real: wr[[i]], imag: wi[[i]]};
        }

        // 右固有ベクトル
        //let mut vectors_r: Matrix<Complex<f64>> = Matrix::zeros([u, u]);
        //for i in 0..u {
        //        vectors_r[[i]] = vr[[i]] + vr[[i]]*im
        //    }
        //}

        EigenResult {values: values, vectors_l: vl, vectors_r: vr}
    }
}


impl Matrix<f64>
where {
    pub fn qr_decomposition(&self) -> (Matrix<f64>, Matrix<f64>) {
        //if self.dims[0] < self.dims[1] {
        //    return None;
        //}

        let mut q = Matrix::identity([self.dims[0], self.dims[0]]);
        let mut r = self.clone();

        let mut jpvt = vec![0; self.dims[1]];
        let mut tau = vec![0f64; min(self.dims[0], self.dims[1])];

        let mut work_size = -1;
        let mut info = 0;

        let (n, m) = (self.dims[0] as i32, self.dims[1] as i32);
        unsafe {
            // calculate work size
            let mut work = vec![0f64; 1];
            //dgeqp3_(
            //    &n,
            //    &m,
            //    r.data.as_mut_ptr(),
            //    &n,
            //    jpvt.as_mut_ptr(),
            //    tau.as_mut_ptr(),
            //    work.as_mut_ptr(),
            //    &mut work_size,
            //    &mut info,
            //);

            //work = vec![0f64; work_size as usize];
            //// QR decomposition with column pivoting
            //dgeqp3_(
            //    &n,
            //    &m,
            //    r.data.as_mut_ptr(),
            //    &n,
            //    jpvt.as_mut_ptr(),
            //    tau.as_mut_ptr(),
            //    work.as_mut_ptr(),
            //    &work_size,
            //    &mut info,
            //);

            // calculate Q from the result of QR decomposition
            let lwork = -1;
            dgeqrf_(
                &n,
                &m,
                r.data.as_mut_ptr(),
                &n,
                tau.as_mut_ptr(),
                work.as_mut_ptr(),
                &lwork,
                &mut info,
            );
            //q_size = work[0] as usize;
            //work_size = q_size as i32;

            //let mut q_data = vec![0f64; q_size];
            let mut work = vec![0f64; work[0] as usize];
            dgeqrf_(
                &n,
                &m,
                r.data.as_mut_ptr(),
                &n,
                tau.as_mut_ptr(),
                work.as_mut_ptr(),
                &lwork,
                &mut info,
            );

            //let q_slice = q.as_slice_mut().unwrap().slice_mut(s![.., ..q_size]);
            //let q_data_slice = q_data.as_slice();
            //q_slice.assign(
            //    &ArrayBase::from_shape_vec((self.dims[0], q_size), q_data_slice.to_vec()).unwrap(),
            //);

            //let r_slice = r.as_slice_mut().unwrap().slice_mut(s![..q_size, ..]);
            //r_slice.triu_mut(0);
        }

        (q, r)
    }

    pub fn lu_decomposition(&self) -> (Matrix<f64>, Matrix<f64>) {
        //if self.dims[0] < self.dims[1] {
        //    return None;
        //}

        //let mut q = Matrix::identity([self.dims[0], self.dims[0]]);
        let mut lu = self.clone();

        let mut ipvt = vec![0; self.dims[1]];

        let mut info = 0;

        let (n, m) = (self.dims[0] as i32, self.dims[1] as i32);
        unsafe {

            dgbtrf_(
                &n,
                &m,
                &n,
                &m,
                lu.data.as_mut_ptr(),
                &n,
                ipvt.as_mut_ptr(),
                &mut info,
            );

        }

        (lu.clone(), lu)
    }
}
