use linear_algebra::{Array, Matrix, Vector};


// $ cargo run --example array_operations

fn v_norm() {
    let v = Vector::<f32>::ones([5]);
    let l1 = v.l1_norm();
    let l2 = v.l2_norm();
    println!("v: {}", v);
    println!("l1: {}", l1);
    println!("l2: {}", l2);
}

fn m_norm() {
    let v = Matrix::<f32>::ones([3, 3]);
    let l1 = v.l1_norm();
    let l2 = v.l2_norm();
    println!("v: {}", v);
    println!("l1: {}", l1);
    println!("l2: {}", l2);
}

fn main() {
    v_norm();
    m_norm();
}
