#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut longest = 0;
        for i in 0..nums.len() {
            let mut min = nums[i];
            let mut max = nums[i];
            let mut early_return = false;
            for (j, &item) in nums.iter().enumerate().skip(i + 1) {
                if item > max {
                    max = item;
                }
                if item < min {
                    min = item;
                }

                if max - min > limit {
                    if j - i > longest {
                        longest = j - i;
                    }
                    early_return = true;
                    break;
                }
            }

            if !early_return && nums.len() - i > longest {
                longest = nums.len() - i;
            }
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
        assert_eq!(
            3,
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0)
        );
    }
}
