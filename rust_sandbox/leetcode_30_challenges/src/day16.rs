/// Valid Parenthesis String
///
/// Given a string containing only three types of characters: '(', ')' and '*', write a function
/// to check whether this string is valid. We define the validity of a string by these rules:
///
///     Any left parenthesis '(' must have a corresponding right parenthesis ')'.
///     Any right parenthesis ')' must have a corresponding left parenthesis '('.
///     Left parenthesis '(' must go before the corresponding right parenthesis ')'.
///     '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
///     An empty string is also valid.
///
/// Example 1:
///
/// Input: "()"
/// Output: True
///
/// Example 2:
///
/// Input: "(*)"
/// Output: True
///
/// Example 3:
///
/// Input: "(*))"
/// Output: True

#[cfg(test)]
struct Solution;

#[cfg(test)]
use std::collections::VecDeque;

#[cfg(test)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut stack = VecDeque::with_capacity(s.len());
        Self::recur(&s, &mut stack)
    }

    fn recur(s: &str, stack: &mut VecDeque<char>) -> bool {
        println!("s: {}, stack: {:?}", s, stack);
        if s.is_empty() {
            println!(
                "s is empty and stack is {}",
                if stack.is_empty() {
                    "empty"
                } else {
                    "not empty"
                }
            );
            stack.is_empty()
        } else {
            let ch = s.chars().next().unwrap();
            let rest = &s[1..];
            match ch {
                '(' => {
                    stack.push_back(ch);
                    let r = Self::recur(rest, stack);
                    stack.pop_back();
                    r
                }
                '*' => {
                    // we can go down potentially 3 different paths from here
                    // treat it like a (
                    stack.push_back('(');
                    let r1 = Self::recur(rest, stack);
                    stack.pop_back();
                    // or treat it like an "empty string"
                    let r2 = Self::recur(rest, stack);
                    // or treat it like an ')'
                    let r3 = if let Some(&'(') = stack.back() {
                        println!("end of stack is (");
                        stack.pop_back();
                        let r = Self::recur(rest, stack);
                        stack.push_back('(');
                        r
                    } else {
                        true
                    };

                    r1 || r2 || r3
                }
                ')' => {
                    if let Some(&'(') = stack.back() {
                        stack.pop_back();
                        let r = Self::recur(rest, stack);
                        stack.push_back('(');
                        r
                    } else {
                        println!("there's no matching ( for )");
                        false
                    }
                }
                _ => panic!(format!("Only ( * or ) are allowed but got {}", ch)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert!(Solution::check_valid_string("(*()".to_string()));
        assert!(Solution::check_valid_string("()".to_string()));
        assert!(Solution::check_valid_string("(*)".to_string()));
        assert!(Solution::check_valid_string("(*))".to_string()));
        assert!(!Solution::check_valid_string("()))".to_string()));
        assert!(!Solution::check_valid_string(
            "*()(())*()(()()((()(()()*)(*(())((((((((()*)(()(*)".to_string()
        ));
        assert!(!Solution::check_valid_string(
            "(()*)(()((())()))(*)((((())*())))()(((()((()(*()))".to_string()
        ));
    }
}
