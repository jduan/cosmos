#[allow(dead_code)]
pub struct Matrix<T> {
    matrix: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    pub fn new(matrix: Vec<Vec<T>>) -> Self {
        let rows = matrix.len();
        if rows == 0 {
            panic!("Matrix needs to have at least one row");
        }
        let first_row = matrix.first().unwrap();
        let cols = first_row.len();
        if cols == 0 {
            panic!("Matrix needs to have at least one column");
        }

        Self { matrix, rows, cols }
    }

    pub fn cell(&self, row: usize, col: usize) -> &T {
        if row >= self.rows || col >= self.cols {
            panic!("Invalid coordinate: ({}, {})", row, col);
        }

        self.matrix.get(row).unwrap().get(col).unwrap()
    }

    pub fn valid_cell(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32
    }

    /// Return valid neighbors (up to 4)
    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = row as i32 + dx;
            let y = col as i32 + dy;
            if x >= 0 && x < self.rows as i32 && y >= 0 && y < self.cols as i32 {
                neighbors.push((x as usize, y as usize));
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let matrix = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![10, 20, 30, 40],
            vec![100, 200, 300, 400],
        ]);

        assert_eq!(3, matrix.rows);
        assert_eq!(4, matrix.cols);
        assert_eq!(&1, matrix.cell(0, 0));
        assert_eq!(&100, matrix.cell(2, 0));

        assert_eq!(vec![(0, 1), (1, 0)], matrix.get_neighbors(0, 0));
        assert_eq!(vec![(1, 1), (2, 0), (0, 0)], matrix.get_neighbors(1, 0));
        assert_eq!(
            vec![(1, 2), (1, 0), (2, 1), (0, 1)],
            matrix.get_neighbors(1, 1)
        );
    }
}
