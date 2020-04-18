use crate::matrix::Matrix;

/// Minimum Path Sum
///
/// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right
/// which minimizes the sum of all numbers along its path.
///
/// Note: You can only move either down or right at any point in time.
///
/// Example:
///
/// Input:
/// [
///   [1,3,1],
///   [1,5,1],
///   [4,2,1]
/// ]
/// Output: 7
/// Explanation: Because the path 1→3→1→1→1 minimizes the sum.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let grid = Matrix::new(grid);
        let mut min_sum = std::i32::MAX;
        Self::recur(&grid, 0, 0, 0, &mut min_sum);

        min_sum
    }

    fn recur(grid: &Matrix<i32>, row: usize, col: usize, current_sum: i32, min_sum: &mut i32) {
        let current_sum = current_sum + grid.cell(row, col);
        // update the max
        if row == grid.rows - 1 && col == grid.cols - 1 && current_sum < *min_sum {
            *min_sum = current_sum;
        }

        // if current_sum is greater than min_sum, don't recur
        if current_sum >= *min_sum {
            return;
        }
        let right_cell = (row as i32, (col + 1) as i32);
        let down_cell = ((row + 1) as i32, col as i32);
        if grid.valid_cell(right_cell.0, right_cell.1) {
            Self::recur(grid, row, col + 1, current_sum, min_sum);
        }
        if grid.valid_cell(down_cell.0, down_cell.1) {
            Self::recur(grid, row + 1, col, current_sum, min_sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1],])
        );

        assert_eq!(
            83,
            Solution::min_path_sum(vec![
                vec![3, 8, 6, 0, 5, 9, 9, 6, 3, 4, 0, 5, 7, 3, 9, 3],
                vec![0, 9, 2, 5, 5, 4, 9, 1, 4, 6, 9, 5, 6, 7, 3, 2],
                vec![8, 2, 2, 3, 3, 3, 1, 6, 9, 1, 1, 6, 6, 2, 1, 9],
                vec![1, 3, 6, 9, 9, 5, 0, 3, 4, 9, 1, 0, 9, 6, 2, 7],
                vec![8, 6, 2, 2, 1, 3, 0, 0, 7, 2, 7, 5, 4, 8, 4, 8],
                vec![4, 1, 9, 5, 8, 9, 9, 2, 0, 2, 5, 1, 8, 7, 0, 9],
                vec![6, 2, 1, 7, 8, 1, 8, 5, 5, 7, 0, 2, 5, 7, 2, 1],
                vec![8, 1, 7, 6, 2, 8, 1, 2, 2, 6, 4, 0, 5, 4, 1, 3],
                vec![9, 2, 1, 7, 6, 1, 4, 3, 8, 6, 5, 5, 3, 9, 7, 3],
                vec![0, 6, 0, 2, 4, 3, 7, 6, 1, 3, 8, 6, 9, 0, 0, 8],
                vec![4, 3, 7, 2, 4, 3, 6, 4, 0, 3, 9, 5, 3, 6, 9, 3],
                vec![2, 1, 8, 8, 4, 5, 6, 5, 8, 7, 3, 7, 7, 5, 8, 3],
                vec![0, 7, 6, 6, 1, 2, 0, 3, 5, 0, 8, 0, 8, 7, 4, 3],
                vec![0, 4, 3, 4, 9, 0, 1, 9, 7, 7, 8, 6, 4, 6, 9, 5],
                vec![6, 5, 1, 9, 9, 2, 2, 7, 4, 2, 7, 2, 2, 3, 7, 2],
                vec![7, 1, 9, 6, 1, 2, 7, 0, 9, 6, 6, 4, 4, 5, 1, 0],
                vec![3, 4, 9, 2, 8, 3, 1, 2, 6, 9, 7, 0, 2, 4, 2, 0],
                vec![5, 1, 8, 8, 4, 6, 8, 5, 2, 4, 1, 6, 2, 2, 9, 7]
            ])
        );
    }
}
