/// Move Zeroes
///
/// Given an array nums, write a function to move all 0's to the end of it while maintaining the
/// relative order of the non-zero elements.
///
/// Example:
///
/// Input: [0,1,0,3,12]
/// Output: [1,3,12,0,0]
///
/// Note:
///
///     You must do this in-place without making a copy of the array.
///     Minimize the total number of operations.

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (0, 0);
        while i < nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }
        // set the remaining of the vector to 0s
        while j < nums.len() {
            nums[j] = 0;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let mut nums = vec![];
        Solution::move_zeroes(&mut nums);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, nums);

        let mut nums = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![0, 0, 0, 0, 0], nums);

        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }
}
