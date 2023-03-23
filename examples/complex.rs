#![allow(non_snake_case)]

use linear_algebra::Complex;

fn cmpl01() {
    println!("compl01");
    let x = Complex{real: 2f64, imag: 3f64};
    println!("x: {}", x);

    println!("ptr: {:p}", &x.real);
    println!("ptr: {:p}", &x.imag);
}

fn cmpl02() {
    println!("compl01");
    let x = vec![
        Complex{real: 2f64, imag: 3f64},
        Complex{real: 2f64, imag: 3f64},
        Complex{real: 2f64, imag: 3f64},
        Complex{real: 2f64, imag: 3f64},
    ];
    println!("x: {:?}", x);
    println!("ptr: {:p}", &x[0].real);
    println!("ptr: {:p}", &x[0].imag);
    println!("ptr: {:p}", &x[1].real);
    println!("ptr: {:p}", &x[1].imag);
    println!("ptr: {:p}", &x[2].real);
    println!("ptr: {:p}", &x[2].imag);
    println!("ptr: {:p}", &x[3].real);
    println!("ptr: {:p}", &x[3].imag);
}

fn main() {
    cmpl01();
    cmpl02();
}
