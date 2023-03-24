#![allow(non_snake_case)]

// $ cargo run --example array_mul
//
use linear_algebra::{Array, Matrix, Vector, Transpose};
use linear_algebra::Complex;

fn mul01() {
    let A = Array::new([3; 2], 2.0);
    let B = Array::new([3; 2], 3.0);
    println!("A: {}", A);
    println!("B: {}", A);
    println!("A*B: {}", A*B);
}

fn mul02() {
    let A = Array::new([3; 3], 2.0);
    println!("{}", A);
}

fn mul03() {
    let A: Array<f64, 3> = Array::zeros([3; 3]);
    let B: Array<f64, 3> = Array::identity([3; 3]);
    println!("{}", A);
    println!("{}", B);
}

fn mul04() {
    let A: Array<f64, 2> = Array::rand([5, 2]);
    println!("{}", A);
}

fn mul05() {
    let A: Array<f64, 2> = Array::rand([5, 2]);
    let B: Array<f64, 1> = Array::rand([2]);
    println!("{}", A);
    println!("{}", B);
    println!("{}", A * B);
}

fn mul06() {
    let A: Array<f64, 2> = Array::ones([3, 3]);
    let b = 4.0;
    println!("{}", A);
    println!("{}", b);
    println!("{}", A.clone() * b);
    println!("{}", b * A.clone());
}

fn mul07() {
    let A: Array<Complex<f64>, 2> = Array::rand([3, 3]);
    let b = Complex {real: 3.0, imag: 0.0};
    println!("{}", A);
    println!("{}", b);
    println!("{}", A.clone() * b);
    println!("{}", b * A.clone());
}

fn mul08() {
    let a: Vector<f64> = Array::rand([3]);
    let b: Vector<f64> = Array::rand([3]);
    println!("{}", a);
    println!("{}", b.clone().transpose());
    println!("{}", a * b.transpose());
}

fn index_00() {
    let mut A: Array<f64, 3> = Array::ones([3; 3]);
    A.data[26] = 10.0;
    A.data[2] = 3.0;
    A.data[5] = 5.0;
    A.data[5] = 5.0;
    println!("{:?} == 3", A[[0, 0, 2]]);
    println!("{:?} == 5", A[[0, 1, 2]]);
    println!("{:?} == 10", A[[2, 2, 2]]);
}


fn main() {
    mul01();
    mul02();
    mul03();
    mul04();
    mul05();
    mul06();
    mul07();
    mul08();
    index_00();
}
