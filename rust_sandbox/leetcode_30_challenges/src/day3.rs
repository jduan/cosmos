#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        if nums.is_empty() {
            return 0;
        } else if nums.len() == 1 {
            return nums[0];
        }

        // If all the nums are negative, return the largest one
        let all_negative = nums.iter().all(|num| *num < 0);
        if all_negative {
            return *nums.iter().max().unwrap();
        }

        let mut current_sum = 0;
        let mut best_sum = i32::min_value();

        for x in nums {
            current_sum = max(current_sum + x, 0);
            best_sum = max(best_sum, current_sum);
        }

        best_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(0, Solution::max_sub_array(vec![]));
        assert_eq!(-1, Solution::max_sub_array(vec![-1]));
        assert_eq!(-1, Solution::max_sub_array(vec![-2, -1]));
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
}
