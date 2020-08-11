use std::collections::HashMap;

/// Longest Substring Without Repeating Characters
///
/// Given a string, find the length of the longest substring without repeating characters.
///
/// Example 1:
///
/// Input: "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
/// Example 2:
///
/// Input: "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    /// We iterate through the string and maintain a map that maps char to index.
    /// When we see the first repeating character, we know the first index of the character.
    /// Then we know all the characters to the right of that index are unique. We start
    /// searching for another unique substring from there.
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut starting_idx = 0;
        let mut max_length = 0;

        for (idx, char) in s.chars().enumerate() {
            if let Some(original_idx) = map.get(&char) {
                if *original_idx >= starting_idx {
                    let length = idx - starting_idx;
                    if length > max_length {
                        max_length = length;
                    }
                    starting_idx = *original_idx + 1;
                }
            }
            // always update the index of the char to the latest
            map.insert(char, idx);
        }

        // We may not have found any repeating characters at all.
        let length = s.len() - starting_idx;
        if length > max_length {
            max_length = length;
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }
}
