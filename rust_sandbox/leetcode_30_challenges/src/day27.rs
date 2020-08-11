use std::cmp::min;
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix.first().unwrap().len();
        if cols == 0 {
            return 0;
        }
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let mut max = 0;

        #[allow(clippy::needless_range_loop)]
        for row in 0..rows {
            for col in 0..cols {
                let cell = matrix[row][col];
                if cell == '0' {
                    cache.insert((row, col), 0);
                } else {
                    let left = {
                        if col == 0 {
                            0
                        } else {
                            *cache.get(&(row, col - 1)).unwrap()
                        }
                    };
                    let up = {
                        if row == 0 {
                            0
                        } else {
                            *cache.get(&(row - 1, col)).unwrap()
                        }
                    };
                    let cross = {
                        if col == 0 || row == 0 {
                            0
                        } else {
                            *cache.get(&(row - 1, col - 1)).unwrap()
                        }
                    };

                    let current_max = min(min(left, up), cross) + 1;
                    cache.insert((row, col), current_max);
                    if current_max > max {
                        max = current_max;
                    }
                }
            }
        }

        max.pow(2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_square() {
        assert_eq!(
            4,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ])
        );
    }
}
