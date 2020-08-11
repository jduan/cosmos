/// Backspace String Compare
///
/// Given two strings S and T, return if they are equal when both are typed into empty text
/// editors. # means a backspace character.
///
/// Example 3:
///
/// Input: S = "a##c", T = "#a#c"
/// Output: true
/// Explanation: Both S and T become "c".
///
/// Example 4:
///
/// Input: S = "a#c", T = "b"
/// Output: false
/// Explanation: S becomes "c" while T becomes "b".

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        let vec1 = Solution::helper(s);
        let vec2 = Solution::helper(t);

        vec1 == vec2
    }

    #[allow(dead_code)]
    fn helper(s: String) -> Vec<char> {
        let mut current = 0;
        let mut stack_pointer = 0;
        let mut chars: Vec<char> = s.chars().collect();
        while current < chars.len() {
            if chars[current] == '#' {
                if stack_pointer > 0 {
                    stack_pointer -= 1;
                }
            } else {
                chars[stack_pointer] = chars[current];
                stack_pointer += 1;
            }
            current += 1;
        }
        chars[..stack_pointer].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
        assert!(Solution::backspace_compare(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }

    #[test]
    fn test_helper() {
        assert_eq!(Solution::helper("a##c".to_string()), vec!['c']);
        assert_eq!(Solution::helper("ab#c".to_string()), vec!['a', 'c']);
        assert_eq!(Solution::helper("ad#c".to_string()), vec!['a', 'c']);
        assert_eq!(Solution::helper("ab##".to_string()), vec![]);
        assert_eq!(Solution::helper("c#d#".to_string()), vec![]);
    }
}
