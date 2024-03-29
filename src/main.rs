#[cfg(not(feature="cuda"))]
fn main() {
    use linear_algebra::{Complex, Matrix, Vector};
    let n = 100;
    let x = Matrix::<f64>::rand([n, n]);
    let (l, u) = x.lu();
    println!("l: {}", l[[10, 10]]);
    println!("u: {}", u[[10, 10]]);
}














#[cfg(feature="cuda")]
fn main() {
    use linear_algebra::{Complex, Matrix, Vector};
    use linear_algebra::{CPU,GPU};
    let n = 3;
    let x = Matrix::<f32>::rand([n, n]);
    let y = Matrix::<f32>::rand([n, n]);
    let cx = x.gpu();
    let cy = y.gpu();
    let cz = cx * cy;
    let z = cz.unwrap().cpu();
    println!("z: {}", z);
}
