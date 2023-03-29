#![allow(non_snake_case)]
use linear_algebra::{Array, Matrix, Vector};

fn slice01() {
    let A = Matrix::<f64>::rand([3, 3]);
    let B = Matrix::<f64>::zeros([2, 2]);
    println!("A: {}", A);
    let S = A.view(1..3, 1..3);
    let Asub = S.subarray();
    println!("S: {:?}", S);
    let A2 = S.set_subarray(B);
    println!("Asub: {}", Asub);
    println!("A: {}", A2);
}

fn submatrix() {
    println!("submatrix");
    let A = Matrix::<f64>::rand([3, 3]);
    println!("A: {}", A);
}


fn submatrix2() {
    println!("submatrix");
    let A = Matrix::<f64>::rand([5, 5]);
    println!("A: {}", A);
    let rows = (0..3).chain(4..5);
    let B = A.clone().view2(rows.clone(), rows).subarray();
    println!("Asub: {}", B);
    let B = A.clone().submatrix(0, 0).subarray();
    println!("Asubmat: {}", B);
}


fn main() {
    slice01();
    submatrix();
    submatrix2();
}
