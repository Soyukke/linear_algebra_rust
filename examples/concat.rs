#![allow(non_snake_case)]


// $ cargo run --example concat

use linear_algebra::{Array, Matrix, Vector};

fn cat00() {
    let a = Vector::<f64>::rand([3]);
    let b = Vector::<f64>::rand([2]);
    let c = a.vcat(&b);
    println!("c {}", c);

    let a = Vector::<f64>::rand([3]);
    let b = Vector::<f64>::rand([3]);
    let c = a.hcat(&b);
    println!("c {}", c);

}

fn cat01() {
    let a = Matrix::<f64>::rand([3, 3]);
    let b = Matrix::<f64>::rand([2, 3]);
    let c = a.vcat(&b);
    println!("c {}", c);

    let a = Matrix::<f64>::rand([3, 3]);
    let b = Matrix::<f64>::rand([3, 2]);
    let c = a.hcat(&b);
    println!("c {}", c);

}

fn cat02() {
    let vs = vec![
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([2]),
        Vector::<f64>::rand([1]),
        Vector::<f64>::rand([4]),
    ];

    let c = vs.into_iter().reduce(|a, b| a.vcat(&b)).unwrap();
    println!("c: {}", c);
}

fn cat03() {
    let vs = vec![
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
        Vector::<f64>::rand([3]),
    ];

    let c = vs.into_iter().map(|v| v.to_mat())
        .into_iter().reduce(|a, b| a.hcat(&b)).unwrap();
    println!("c: {}", c);
}


fn main() {
    cat00();
    cat01();
    cat02();
    cat03();
}
