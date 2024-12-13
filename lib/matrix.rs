struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    fn new(rows: usize, cols: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default_value; rows * cols],
            rows,
            cols,
        }
    }

    fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        return &self.data[row * self.cols + col];
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        return &mut self.data[row * self.cols + col];
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix_create() {
        let m = Matrix::<i32>::new(10, 10, 1);
        assert_eq!(m.rows, 10);
        assert_eq!(m.cols, 10);
        assert_eq!(m.get(0, 0), &1);
    }

    #[test]
    fn test_matrix_set() {
        let mut m = Matrix::<i32>::new(10, 10, 1);
        m.set(0, 0, 10);
        assert_eq!(m.get(0, 0), &10);
    }

    #[test]
    fn test_matrix_get_mut() {
        let mut m = Matrix::<i32>::new(10, 10, 1);
        *m.get_mut(0, 0) = 2;
        assert_eq!(m.get(0, 0), &2);
    }
}
