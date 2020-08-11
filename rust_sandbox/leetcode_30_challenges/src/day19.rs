/// Search in Rotated Sorted Array
//
// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
//
// (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
//
// You are given a target value to search. If found in the array return its index, otherwise return -1.
//
// You may assume no duplicate exists in the array.
//
// Your algorithm's runtime complexity must be in the order of O(log n).
//
// Example 1:
//
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
//
// Example 2:
//
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1
use std::cmp::Ordering;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_rotated(&nums[0..nums.len()], 0, target)
    }

    fn search_rotated(nums: &[i32], start_index: usize, target: i32) -> i32 {
        println!("search_rotated: {:?}", nums);
        if nums.is_empty() {
            println!("nums is empty now");
            return -1;
        }

        let low: usize = 0;
        let high = nums.len() - 1;
        let mid = (low + high) / 2;
        println!("mid: {}", mid);
        if nums[mid] == target {
            println!("found target at {}", mid);
            return (start_index + mid) as i32;
        }

        let left_ret = if low < mid {
            let new_nums = &nums[low..mid];
            if nums[low] < nums[mid] {
                Self::binary_search(new_nums, start_index, target)
            } else {
                Self::search_rotated(new_nums, start_index, target)
            }
        } else {
            -1
        };
        if left_ret != -1 {
            return left_ret;
        }

        let right_ret = if mid < high {
            let new_nums = &nums[mid + 1..=high];
            if nums[mid] < nums[high] {
                Self::binary_search(new_nums, start_index + mid + 1, target)
            } else {
                Self::search_rotated(new_nums, start_index + mid + 1, target)
            }
        } else {
            -1
        };
        if right_ret != -1 {
            return right_ret;
        }

        -1
    }

    fn binary_search(nums: &[i32], start_index: usize, target: i32) -> i32 {
        println!("binary_search: {:?}, start_index: {}", nums, start_index);
        if nums.is_empty() {
            return -1;
        }

        if target < nums[0] || target > nums[nums.len() - 1] {
            return -1;
        }

        let mid = nums.len() / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => (start_index + mid) as i32,
            Ordering::Less => {
                if mid < nums.len() {
                    Self::binary_search(&nums[mid + 1..nums.len()], start_index + mid + 1, target)
                } else {
                    -1
                }
            }
            Ordering::Greater => {
                if mid > 0 {
                    Self::binary_search(&nums[0..mid], start_index, target)
                } else {
                    -1
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(2, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 6));
    }
}
