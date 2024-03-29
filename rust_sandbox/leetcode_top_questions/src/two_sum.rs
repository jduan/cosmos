use std::collections::HashMap;

/// Given an array of integers, return indices of the two numbers such that they add up to a
/// specific target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same
/// element twice.
///
/// Example:
///
/// Given nums = [2, 7, 11, 15], target = 9,
///
/// Because nums[0] + nums[1] = 2 + 7 = 9,
/// return [0, 1].

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let other = target - *num;
            match map.get(&other) {
                Some(other_idx) => return vec![*other_idx as i32, idx as i32],
                None => {
                    map.insert(num, idx);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
