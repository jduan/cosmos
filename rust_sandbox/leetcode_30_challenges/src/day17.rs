use crate::matrix::Matrix;
use std::collections::HashSet;
use std::collections::VecDeque;
/// Number of Islands
///
/// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island
/// is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
/// You may assume all four edges of the grid are all surrounded by water.
///
/// Example 1:
///
/// Input:
/// 11110
/// 11010
/// 11000
/// 00000
///
/// Output: 1
///
/// Example 2:
///
/// Input:
/// 11000
/// 11000
/// 00100
/// 00011
///
/// Output: 3

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let grid = Matrix::new(grid);
        let mut islands = 0;
        let mut visited = HashSet::new();

        println!("before loop");
        for i in 0..grid.rows {
            for j in 0..grid.cols {
                let cell = (i, j);
                let value = grid.cell(i, j);
                if !visited.contains(&cell) && value == &'1' {
                    println!("Visiting cell {:?}", cell);
                    let mut queue = VecDeque::new();
                    queue.push_back(cell);
                    visited.insert(cell);
                    Self::visit(&grid, &mut queue, &mut visited);
                    islands += 1;
                }
            }
        }

        islands
    }

    fn visit(
        grid: &Matrix<char>,
        queue: &mut VecDeque<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            println!("head of queue: ({}:{})", i, j);
            for neighbor in grid.get_neighbors(i, j) {
                let value = grid.cell(neighbor.0, neighbor.1);
                if !visited.contains(&neighbor) && value == &'1' {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        println!("testing");
        assert_eq!(
            1,
            Solution::num_islands(vec![
                "11110".chars().collect(),
                "11010".chars().collect(),
                "11000".chars().collect(),
                "00000".chars().collect(),
            ])
        );
        assert_eq!(
            3,
            Solution::num_islands(vec![
                "11000".chars().collect(),
                "11000".chars().collect(),
                "00100".chars().collect(),
                "00011".chars().collect(),
            ])
        );
    }
}
