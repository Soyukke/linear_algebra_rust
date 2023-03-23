#![allow(non_snake_case)]

use linear_algebra::Array;

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
    index_00();
}
