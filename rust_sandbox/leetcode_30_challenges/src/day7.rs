/// Given an integer array arr, count element x such that x + 1 is also in arr.
///
/// If there're duplicates in arr, count them seperately.
///
/// Example 1:
///
/// Input: arr = [1,2,3]
/// Output: 2
/// Explanation: 1 and 2 are counted cause 2 and 3 are in arr.
///
/// Example 2:
///
/// Input: arr = [1,1,3,3,5,5,7,7]
/// Output: 0
/// Explanation: No numbers are counted, cause there's no 2, 4, 6, or 8 in arr.

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        use std::collections::BTreeSet;

        let set: BTreeSet<&i32> = arr.iter().collect();
        arr.iter().fold(0, |mut acc, i| {
            let num = i + 1;
            if set.contains(&num) {
                acc += 1;
            }

            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(2, Solution::count_elements(vec![1, 2, 3]));
    }
}
