pub mod complex;
pub mod vector;
pub mod matrix;
pub mod basic_trait;

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::matrix::{*};
    use crate::basic_trait::One;

    // cargo test mul_comp_mat -- --nocapture
    #[test]
    fn mul_comp_mat() {
        let mut n = Matrix::<Complex<f64>, 2, 2>::new();
        n[0][0] = Complex::new(3.0, 2.0);
        let mut l = Matrix::<Complex<f64>, 2, 2>::new();
        l[0][0] = Complex::new(3.0, -2.0);
        println!("{:?}", n*l);
        assert_eq!(2 + 2, 4);
    }

    // cargo test identity_matrix -- --nocapture
    #[test]
    fn identity_matrix() {
        let n = Matrix::<f64, 3, 3>::one();
        println!("{}", n);
        assert_eq!(2 + 2, 4);
    }

}
