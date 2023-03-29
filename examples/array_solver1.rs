#![allow(non_snake_case)]
use linear_algebra::{Array, Matrix, Vector, Transpose};

fn qr01() {
    let A = Matrix::<f64>::rand([3, 3]);
    //A.dot(&A);
    println!("A: {}", A);
    let (q, r) = A.qr();
    println!("q: {}", q);
    println!("r: {}", r);
    println!("q*q: {}", q.clone().transpose() * q.clone());
    println!("qr: {}", q.clone() * r.clone());
    println!("A - qr: {}", A - q * r);
}


fn main() {
    qr01();

}
