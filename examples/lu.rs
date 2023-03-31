#![allow(non_snake_case)]


// $ cargo run --example concat

use linear_algebra::{Array, Matrix, Vector};


fn lu01() {
    let a = Matrix::<f64>::rand([4, 4]);
    println!("a {}", a);
    let (l, u) = a.lu();
    println!("l {}", l);
    println!("u {}", u);
    println!("a - l*u {}", a - l*u);
}

fn lu02() {
    let n = 200;
    let a = Matrix::<f64>::rand([n, n]);
    println!("initialize ok");
    let (l, u) = a.lu();
    println!("a - l*u {}", a - l*u);
    println!("lu decompose ok");
}

fn main() {
    lu01();
    lu02();
}
