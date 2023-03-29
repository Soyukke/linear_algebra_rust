#![allow(non_snake_case)]

// $ cargo run --example array_lapack --features=blas

use linear_algebra::Array;
use linear_algebra::lapack_ffi::*;


fn mul01() {
    let A = Array::new([3; 2], 2.0);
    let B = Array::new([3; 2], 3.0);
    println!("A: {}", A);
    println!("B: {}", B);
    println!("A*B: {}", A*B);
}

fn mul02() {
    let A: Array<f32, 2>  = Array::rand([3; 2]);
    let B: Array<f32, 2>= Array::identity([3; 2]);
    println!("A: {}", A);
    println!("B: {}", B);
    println!("A*B: {}", A*B);
}

fn mul03() {
    let A: Array<f64, 2>  = Array::rand([3; 2]);
    let B: Array<f64, 2>= Array::identity([3; 2]);
    println!("A: {}", A);
    println!("B: {}", B);
    println!("A*B: {}", A*B);
}

fn eigen01() {
    let mut A: Array<f64, 2>  = Array::rand([3; 2]);
    println!("A: {}", A);
    let result = A.eigen();
    println!("values: {}", result.values);
}


fn qr01() {
    let mut A: Array<f64, 2>  = Array::rand([3; 2]);
    println!("A: {}", A);
    //let result = qr_decomposition(A);
    //println!("values: {}", result);
}

fn main() {
    mul01();
    mul02();
    mul03();
    eigen01();
    qr01();
}

