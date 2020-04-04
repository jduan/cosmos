/// Single Number
///
/// Given a non-empty array of integers, every element appears twice except for one. Find that single one.
///
/// Note:
///
/// Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
///
/// Example 1:
///
/// Input: [2,2,1]
/// Output: 1
///
/// Example 2:
///
/// Input: [4,1,2,1,2]
/// Output: 4

#[allow(dead_code)]
pub fn find_single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, b| a ^ *b)
}

#[cfg(test)]
mod tests {
    use crate::day1::find_single_number;

    #[test]
    fn test_find_single_number() {
        let nums = vec![4, 1, 2, 1, 2];
        let single_number = find_single_number(nums);
        assert_eq!(4, single_number);

        let nums = vec![1, 2, 1];
        let single_number = find_single_number(nums);
        assert_eq!(2, single_number);
    }
}
