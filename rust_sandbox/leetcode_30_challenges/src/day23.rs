/// Bitwise AND of Numbers Range
///
/// Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in
/// this range, inclusive.
///
/// Example 1:
///
/// Input: [5,7]
/// Output: 4
///
/// Example 2:
///
/// Input: [0,1]
/// Output: 0

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if m == std::i32::MAX {
            return m;
        }

        let mut result = m;
        for i in (m + 1)..=n {
            result &= i;
        }

        result
    }

    // faster way
    // The trick here is that :
    // Bitwise-AND of any two numbers will always produce a number less than or equal to the smaller number.
    //
    // Consider the following example:
    //
    // 									12 ---- 1100
    // 									11 ---- 1011
    // 									10 ---- 1010
    // 									9  ---- 1001
    // 									8  ---- 1000
    // 									7  ---- 0111
    // 									6  ---- 0110
    // 									5  ---- 0101
    //
    // Desired Range: [5,12]
    //
    // Starting from 12, the loop will first do
    // 12 & 11 = 8
    //
    // Next iteration, the loop will do
    // 8 & 7 = 0
    //
    // why did we skip anding of 10,9? Because even if we did so, the result would eventually be anded with 8 whose value would be lesser than equal to 8.
    //
    // Hence, you start from the range end and keep working your way down the range till you reach the start.
    //
    //  while(n>m)
    //            n = n & n-1;
    //  return m&n;
    pub fn range_bitwise_and2(m: i32, n: i32) -> i32 {
        let mut n2 = n;
        while m < n2 {
            n2 &= n2 - 1;
        }

        m & n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 1));
        assert_eq!(
            2147483647,
            Solution::range_bitwise_and(2147483647, 2147483647)
        );

        assert_eq!(4, Solution::range_bitwise_and2(5, 7));
        assert_eq!(0, Solution::range_bitwise_and2(0, 1));
        assert_eq!(
            2147483647,
            Solution::range_bitwise_and2(2147483647, 2147483647)
        );
    }
}
