#![allow(non_snake_case)]


// $ cargo run --example array_add

use linear_algebra::{Array, Matrix, Vector};


fn mul00() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Vector::<f32>::ones([3]);
    let C = A * B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Vector::<f32>::ones([3]);
    let C = &A * &B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Vector::<f32>::ones([3]);
    let C = &A * B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Vector::<f32>::ones([3]);
    let C = A * &B;
}

fn mul01() {
    let A = Matrix::<f32>::rand([3, 3]);
    println!("A: {}", A);
    let B = Matrix::<f32>::ones([3, 3]);
    let C = A * &B;
    println!("B: {}", B);
    println!("C: {}", C);
 
}

fn mul02() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::ones([3, 3]);
    let C = &A * &B;
    println!("A: {}", A);
    println!("B: {}", B);
    println!("C: {}", C);
 
}

fn mul03() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::ones([3, 3]);
    println!("B: {}", B);
    let C = &A * B;
    println!("A: {}", A);
    println!("C: {}", C);
 
}

fn mul04() {
    let A = Matrix::<f32>::rand([3, 3]);
    println!("A: {}", A);
    let B = Matrix::<f32>::ones([3, 3]);
    println!("B: {}", B);
    let C = A * B;
    println!("C: {}", C);
 
}

fn add00() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = &A + &B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = A + B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = &A + B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = A + &B;



}


fn add05() {
    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    println!("A: {}", A);
    println!("B: {}", B);
    let C = A + B;
    println!("C: {}", C);
}

fn add06() {
    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    println!("A: {}", A);
    println!("B: {}", B);
    let C = &A + B;
    println!("C: {}", C);
 
}

fn add07() {
    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    println!("A: {}", A);
    println!("B: {}", B);
    let C = A + &B;
    println!("C: {}", C);
 
}

fn add08() {
    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    println!("A: {}", A);
    println!("B: {}", B);
    let C = &A + &B;
    println!("C: {}", C);
}

fn sub00() {
    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = &A - &B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = A - B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = &A - B;

    let A = Matrix::<f32>::rand([3, 3]);
    let B = Matrix::<f32>::rand([3, 3]);
    let C = A - &B;

    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    let C = &A - &B;

    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    let C = A - B;

    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    let C = &A - B;

    let A = Vector::<f32>::rand([3]);
    let B = Vector::<f32>::rand([3]);
    let C = A - &B;

}

fn main() {
    mul00();
    mul01();
    mul02();
    mul03();
    mul04();
    add00();
    add05();
    add06();
    add07();
    add08();
    sub00();
}
