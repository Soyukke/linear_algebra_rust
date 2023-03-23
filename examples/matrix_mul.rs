#![allow(non_snake_case)]

use linear_algebra::Matrix;

fn mul01() {
    let A = Matrix::<f64>::identity(3, 3);
    let B = Matrix::<f64>::rand(3, 5);
    println!("{}", A);
    println!("{}", B);
    println!("{}", A*B);
}

fn mul02() {
    let A = Matrix::<i32>::ones(4, 3);
    let B = Matrix::new(3, 5, 3_i32);
    println!("{}", A);
    println!("{}", B);
    println!("{}", A*B);
}

fn main() {
    mul01();
    mul02();
}
