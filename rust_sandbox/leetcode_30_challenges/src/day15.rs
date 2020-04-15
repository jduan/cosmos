/// Product of Array Except Self
///
/// Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].
///
/// Example:
///
/// Input:  [1,2,3,4]
/// Output: [24,12,8,6]
///
/// Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.
///
/// Note: Please solve it without division and in O(n).
///
/// Follow up:
/// Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut running_product = 1;
        let mut products = vec![1; nums.len()];

        // The idea is that as we iterate through the list, we update the next element by
        // multiplying it with the current running product. The same applies when we iterate
        // right to left.
        for i in 0..(nums.len() - 1) {
            running_product *= nums[i];
            products[i + 1] *= running_product;
        }

        running_product = 1;
        for i in (1..nums.len()).rev() {
            running_product *= nums[i];
            products[i - 1] *= running_product;
        }

        products
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }
}
