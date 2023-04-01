// cargo run --example lpk --features blas

use linear_algebra::{Matrix, Vector, ArrayValue, Transpose, One};

fn qr() {
    let x = Matrix::<f64>::rand([5, 5]);
    let (q, r) = x.qr_decomposition();
    println!("{}", q);
    println!("{}", r);
}

fn lu() {
    let x = Matrix::<f64>::rand([10, 10]);
    let (p, l, u) = x.lu_decomposition();
    println!("x: {}", x);
    println!("P: {}", p);
    println!("L: {}", l);
    println!("U: {}", u);
    println!("x - lu: {}", &x - &p.transpose()*&l*&u);
}

fn permut() {
    let a = Matrix::<f64>::rand([4, 4]);
    let m = Matrix::<f64>::mutation_matrix(vec![1, 1, 3, 3]);
    println!("m: {}", m);
    println!("a: {}", a);
    println!("m*a: {}", &m*&a);
}

fn lu2() {
    let n = 10000;
    let x = Matrix::<f64>::rand([n, n]);
    let (p, l, u) = x.lu_decomposition();
    //println!("x: {}", x);
    //println!("P: {}", p);
    //println!("L: {}", l);
    //println!("U: {}", u);
}

fn main() {
    //qr();
    //lu();
    lu2();
    //permut();
}
