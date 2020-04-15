/// Perform String Shifts
///
/// You are given a string s containing lowercase English letters, and a matrix shift, where
/// shift[i] = [direction, amount]:
///
///     direction can be 0 (for left shift) or 1 (for right shift).
///     amount is the amount by which string s is to be shifted.
///     A left shift by 1 means remove the first character of s and append it to the end.
///     Similarly, a right shift by 1 means remove the last character of s and add it to the beginning.
///
/// Return the final string after all operations.
///
///  
///
/// Example 1:
///
/// Input: s = "abc", shift = [[0,1],[1,2]]
/// Output: "cab"
/// Explanation:
/// [0,1] means shift to left by 1. "abc" -> "bca"
/// [1,2] means shift to right by 2. "bca" -> "cab"

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut total_amount: i32 = 0;
        for shifts in shift {
            let direction = shifts.get(0).unwrap();
            let amount = shifts.get(1).unwrap();
            match *direction {
                0 => total_amount -= amount,
                1 => total_amount += amount,
                _ => panic!("Invalid direction: {}", *direction),
            }
        }
        println!("total_mount: {}", total_amount);
        total_amount %= s.len() as i32;
        println!("total_mount after remainder: {}", total_amount);
        // always perform right shift
        let final_shift = if total_amount < 0 {
            (total_amount + s.len() as i32) as usize
        } else {
            total_amount as usize
        };
        println!("final_shift: {}", final_shift);

        let idx = s.len() - final_shift;
        let left = &s[..idx];
        let right = &s[idx..];

        let mut result = right.to_string();
        result.push_str(left);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(
            "cab".to_string(),
            Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]])
        );
        assert_eq!(
            "efgabcd".to_string(),
            Solution::string_shift(
                "abcdefg".to_string(),
                vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
            )
        );
        assert_eq!(
            "kmecs".to_string(),
            Solution::string_shift(
                "mecsk".to_string(),
                vec![vec![1, 4], vec![0, 5], vec![0, 4], vec![1, 1], vec![1, 5]]
            )
        );
    }
}
