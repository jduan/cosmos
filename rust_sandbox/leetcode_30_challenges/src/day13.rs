///  Contiguous Array
///
/// Given a binary array (which has only 0s and 1s), find the maximum length of a contiguous
/// subarray with equal number of 0 and 1.
///
/// Example 1:
///
/// Input: [0,1]
/// Output: 2
/// Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
///
/// Example 2:
///
/// Input: [0,1,0]
/// Output: 2
/// Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
///
/// Note: The length of the given binary array will not exceed 50,000.

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    /// In this approach, we make use of a count variable, which is used to store the relative
    /// number of ones and zeros encountered so far while traversing the array. The count variable
    /// is incremented by one for every 1 encountered and decremented by one for every 0
    /// encountered.
    ///
    /// We have two observations:
    /// 1. If the count reaches 0, we know we've encountered equal number of zeros and ones from
    /// the very beginning until the current index of the array.
    /// 2. Every time the same "count" value is encountered, it means that the ones and zeros in
    /// between are neutralized.
    ///
    /// We can use a hash map to maintain the count values to indices.
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut counts: HashMap<i32, usize> = HashMap::new();

        let mut max = 0;
        let mut i = 0;
        let mut count = 0;
        while i < nums.len() {
            match nums[i] {
                1 => count += 1,
                0 => count -= 1,
                _ => panic!("Unexpected input: {}", nums[i]),
            }
            if count == 0 && i > max {
                max = i + 1;
            }
            let entry = counts.entry(count).or_insert(i);
            let local_max = i - *entry;
            if local_max > max {
                max = local_max;
            }

            i += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
        assert_eq!(4, Solution::find_max_length(vec![0, 1, 0, 1]));
        assert_eq!(6, Solution::find_max_length(vec![0, 0, 0, 1, 1, 1,]));
        assert_eq!(6, Solution::find_max_length(vec![0, 1, 1, 1, 0, 0,]));
        assert_eq!(4, Solution::find_max_length(vec![0, 1, 1, 0, 1, 1, 1, 0]));
    }
}
