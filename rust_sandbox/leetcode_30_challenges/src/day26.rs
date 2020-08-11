use std::cmp::max;
use std::collections::HashMap;

/// Longest Common Subsequence
///
/// Given two strings text1 and text2, return the length of their longest common subsequence.
///
/// A subsequence of a string is a new string generated from the original string with some
/// characters(can be none) deleted without changing the relative order of the remaining characters.
/// (eg, "ace" is a subsequence of "abcde" while "aec" is not). A common subsequence of two strings
/// is a subsequence that is common to both strings.
///
/// If there is no common subsequence, return 0.
///
/// Example 1:
///
/// Input: text1 = "abcde", text2 = "ace"
/// Output: 3
/// Explanation: The longest common subsequence is "ace" and its length is 3.
///
/// Example 2:
///
/// Input: text1 = "abc", text2 = "abc"
/// Output: 3
/// Explanation: The longest common subsequence is "abc" and its length is 3.
///
/// Example 3:
///
/// Input: text1 = "abc", text2 = "def"
/// Output: 0
/// Explanation: There is no such common subsequence, so the result is 0.

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut cache: HashMap<(&[u8], &[u8]), i32> = HashMap::new();
        Self::dynamic_programming(&mut cache, &text1.as_bytes(), &text2.as_bytes())
    }

    /// The cache is used to avoid recalculating things.
    fn dynamic_programming<'a>(
        cache: &mut HashMap<(&'a [u8], &'a [u8]), i32>,
        text1: &'a [u8],
        text2: &'a [u8],
    ) -> i32 {
        if cache.contains_key(&(text1, text2)) {
            return *cache.get(&(text1, text2)).unwrap();
        }

        if text1.is_empty() || text2.is_empty() {
            return 0;
        }

        if text1[0] == text2[0] {
            return Self::dynamic_programming(cache, &text1[1..], &text2[1..]) + 1;
        }

        // either we don't use the first letter of text1
        let lcs1 = Self::dynamic_programming(cache, &text1[1..], &text2);
        cache.insert((&text1[1..], &text2), lcs1);
        // or when we use the first letter of text1, we find the first letter of text2
        // that matches it
        let first_letter = text1[0];
        let lcs2 = match Self::find_letter(text2, first_letter) {
            Some(idx) => {
                if idx == text2.len() - 1 {
                    1
                } else {
                    let ret = Self::dynamic_programming(cache, &text1[1..], &text2[(idx + 1)..]);
                    cache.insert((&text1[1..], &text2[(idx + 1)..]), ret);
                    1 + ret
                }
            }
            None => 0,
        };

        max(lcs1, lcs2)
    }

    fn find_letter(text: &[u8], letter: u8) -> Option<usize> {
        for (i, &item) in text.iter().enumerate() {
            if item == letter {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
        );
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
        );
        assert_eq!(
            0,
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string())
        );
        assert_eq!(
            1,
            Solution::longest_common_subsequence("bl".to_string(), "yby".to_string())
        );
        assert_eq!(
            20,
            Solution::longest_common_subsequence(
                "tufgfqlspqpidwrmjexifslkzobcjqvwlevghglynojchkvufqnwxixqnercbabm".to_string(),
                "xuhadmbsbabqzirgrcxazcxypmjebgxyzmlepcdezwbsjkjalgdotcjavojehsvax".to_string()
            )
        );
    }
}
