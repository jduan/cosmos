/// Happy Number
///
/// Write an algorithm to determine if a number is "happy".
///
/// A happy number is a number defined by the following process: Starting with any positive integer,
/// replace the number by the sum of the squares of its digits, and repeat the process until the
/// number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
/// Those numbers for which this process ends in 1 are happy numbers.
///
/// Example:
///
/// Input: 19
/// Output: true
/// Explanation:
/// 12 + 92 = 82
/// 82 + 22 = 68
/// 62 + 82 = 100
/// 12 + 02 + 02 = 1

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn is_happy(n: i32) -> bool {
        let mut visited = std::collections::HashSet::new();
        visited.insert(n);
        let mut next = n;
        loop {
            next = next
                .to_string()
                .split("")
                // split returns an extra leading and trailing whitespace
                .filter(|s| !s.is_empty())
                .fold(0, |a, b| {
                    // println!("char: {}", b);
                    let digit = b.parse::<i32>().unwrap();
                    a + digit * digit
                });
            // println!("next: {}", next);

            if next == 1 {
                return true;
            }

            // We've encountered a loop!
            if !visited.insert(next) {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Solution;

    #[test]
    fn test_hello() {
        assert!(Solution::is_happy(19));
        assert!(Solution::is_happy(7));
    }
}
