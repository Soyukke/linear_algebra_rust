use std::ops::{Add, Mul, AddAssign, Index, IndexMut};

#[derive(Debug, Clone)]
struct Vector<T, const ROWS: usize> {
    data: [T; ROWS],
}

impl<T: Default + Copy, const ROWS: usize> Vector<T, ROWS> {
    fn new() -> Self {
        Self {
            data: [T::default(); ROWS],
        }
    }

    fn from_vec(v: [T; ROWS]) -> Self {
        Self {data: v}
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize> Index<usize> for Vector<T, ROWS> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize> IndexMut<usize> for Vector<T, ROWS> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}



#[derive(Debug, Clone)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    fn nrow(&self) -> usize {
        ROWS
    }

    fn ncol(&self) -> usize {
        COLS 
    }

}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut [T; COLS] {
        &mut self.data[index]
    }
}

impl<T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        result
    }
}

// Tはmulできて、addAssignできて、コピーできて、デフォルトがあるtype
impl<T: Mul<Output = T> + AddAssign + Default + Copy, const ROWS: usize, const COLS: usize> Mul<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                //result.data[i][j] = self.data[i][j] * other.data[i][j];
                let mut x = T::default();
                for k in 0..ROWS {
                    x += self.data[i][k] * other.data[k][j]
                }
                result.data[i][j] = x;
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + AddAssign + Default + Copy, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>> for Matrix<T, ROWS, COLS> {
    type Output = Vector<T, ROWS>;
    fn mul(self, other: Vector<T, COLS>) -> Vector<T, ROWS> {
        // (ROWS, COLS) * (COLS, 1) = (ROWS, 1)
        let mut result = Vector::<T, ROWS>::new();

        for i in 0..ROWS {
            let mut x = T::default();
            for j in 0..COLS {
                x += self.data[i][j] * other.data[j]
            }
            result.data[i] = x;
        }
        result
    }
}




fn main() {
    let mut m = Matrix::<i32, 4, 4>::new();
    m.data[0][0] = 4_i32;
    let mut m2 = m.clone();
    m2.data[0][3] = 3_i32;
    m2.data[2][2] = 1_i32;
    m2.data[3][3] = 1_i32;
    println!("Hello, world!");
    println!("m: {:?}", m);
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
}
