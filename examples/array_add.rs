#![allow(non_snake_case)]

// $ cargo run --example array_add

use linear_algebra::{Array, Matrix, Vector};

fn add01() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::ones([3, 3]);
    println!("{}", A);
    println!("{}", B);
    println!("{}", A + B);
}

fn add02() {
    let A = Vector::<f32>::rand([5]);
    let B = Vector::<f32>::ones([5]);
    println!("{}", A);
    println!("{}", B);
    println!("{}", A + B);
}

fn main() {
    add01();
    add02();
}
