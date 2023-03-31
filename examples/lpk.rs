use linear_algebra::Matrix;

fn qr() {
    let x = Matrix::<f64>::rand([5, 5]);
    let (q, r) = x.qr_decomposition();
    println!("{}", q);
    println!("{}", r);
}

fn main() {
    qr();
}
