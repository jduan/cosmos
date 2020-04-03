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
