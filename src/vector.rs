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


