pub mod complex;
pub mod vector;
pub mod matrix;
pub mod basic_trait;

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::matrix::{*};

    #[test]
    fn mul_comp_mat() {
        let mut n = Matrix::<Complex<f64>, 2, 2>::new();
        n[0][0] = Complex::new(3.0, 2.0);
        let mut l = Matrix::<Complex<f64>, 2, 2>::new();
        l[0][0] = Complex::new(3.0, -2.0);
        println!("{:?}", n*l);
        assert_eq!(2 + 2, 4);
    }
}
