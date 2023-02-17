use linear_algebra::complex::{*};
use linear_algebra::matrix::{*};
use linear_algebra::vector::{*};

fn main() {
    let mut m = Matrix::<i32, 4, 4>::new();
    m.data[0][0] = 4_i32;
    let mut m2 = m.clone();
    m2.data[0][3] = 3_i32;
    m2.data[2][2] = 1_i32;
    m2.data[3][3] = 1_i32;
    println!("Hello, world!");
    println!("m: {:?}", m);
    println!("m: {}", &m);
    println!("m2: {:?}", m2);
    println!("m2.nrow: {:?}", m2.nrow());
    println!("m2.nrow: {:?}", m2.ncol());
    println!("m2[0]: {:?}", m2[0]);
    println!("m2[0][0]: {:?}", m2[0][0]);
    println!("m + m2: {:?}", m.clone() + m2.clone());
    println!("m * m2: {:?}", m.clone() * m2.clone());


    let mut v = Vector::<i32, 4>::new();
    let mut v2 = Vector::from_vec([0, 1, 200, 2]);
    v[0] = 3;
    v[1] = 3;
    v[2] = 2;
    println!("v: {:?}", v);
    println!("v[0]: {:?}", v[0]);
    println!("v[0]: {:?}", v2[0]);
    println!("m2 * v2: {:?}", m2 * v2);

    let mut c1 = Complex::<i32>::default();
    c1.imag = 2;
    let c2 = Complex::<f32>::default();
    let mut c3 = Complex::<i32>::default();
    c3.real = 4;
    println!("complex c1: {:?}", c1);
    println!("complex c2: {:?}", c2);
    println!("complex c1 + c3: {:?}", c1 + c3);
    println!("complex c1 + c3: {:?}", c1 + c3);
    println!("complex c1 * 4 : {:?}", c1 * 4);
    println!("complex 4 * c1 : {:?}", 4 * c1);
    println!("complex conjugate c1 : {:?}", c1.conj());
    println!("complex conjugate c3 : {:?}", c3.conj());
}
