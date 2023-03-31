#![allow(non_snake_case)]

use linear_algebra::Matrix;
use itertools::iproduct;
use rayon::prelude::*;

fn mul01() {
    let a = Matrix::<f64>::rand([3, 3]);
    let mut z = Matrix::<f64>::rand([3, 3]);
    a.mulr(&a);

    let a = 0..3;
    let b = 0..2;
    let c = a.zip(b).for_each(|(a, b)| 
        println!("a, b: {}, {}", a, b)
    );
    //for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
    //    println!("i, j, k: {}, {}, {}", i, j, k)
    //}

    //(0..10).into_par_iter().for_each(|k| {
    //    (k..10).into_par_iter().for_each(|i| {
    //        // do something
    //        println!("{}", k + i);
    //    });
    //});

    let mut vec: Vec<Vec<f32>> = vec![
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
        vec![0.0; 10],
    ];

    vec.par_iter_mut().enumerate().for_each(|(k, row)| {
        for i in k..10 {
            row[i] = k as f32 + i as f32;
        }
    });
    println!("vec: {:?}", vec);

    //let mut v = z.data;

    //z.into_iter().for_each(|x|
    //    {
    //        println!("{}", x);
    //    }
    //);
    //z.iter_mut().for_each(|x| *x=3.0);
    z.data.iter_mut().for_each(|x| *x=3.0);
    println!("z: {}", z);
    //(0..10).into_par_iter().for_each(
    //    |k|
    //    v[k] = 3.0
    //);
    //


}

fn mul02() {
    let n = 5;
    let mut x = Matrix::<f64>::rand([n, n]);
    let (l, r) = x.lu_r();
    //let (l, r) = x.lu_r();
    println!("L: {}", l);
    println!("R: {}", r);
    println!("X - LR: {}", x - l*r);
}

fn mul03() {
    let n = 1000;
    let mut l = Matrix::<f64>::rand([n, n]);
    let mut u = Matrix::<f64>::rand([n, n]);
    let z = l * u;
}



fn main() {
    //mul01();
    //mul02();
    mul02();
}
