#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last_one = None;

        // 1 0 0 1 0 1
        for (idx, num) in nums.iter().enumerate() {
            if *num == 1 {
                if last_one.is_none() {
                    last_one = Some(idx);
                } else {
                    if idx - last_one.unwrap() - 1 < k as usize {
                        println!("idx: {}", idx);
                        println!("last_one: {:?}", last_one);
                        return false;
                    }
                    last_one = Some(idx);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_length_apart() {
        assert_eq!(
            true,
            Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
        );
        assert_eq!(false, Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
        assert_eq!(true, Solution::k_length_apart(vec![1, 1, 1, 1, 1], 0));
    }
}
