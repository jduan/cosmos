#[allow(dead_code)]
pub fn find_single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, b| a ^ *b)
}

#[cfg(test)]
mod tests {
    use crate::day1::find_single_number;

    #[test]
    fn test_find_single_number() {
        let nums = vec![4, 1, 2, 1, 2];
        let single_number = find_single_number(nums);
        assert_eq!(4, single_number);

        let nums = vec![1, 2, 1];
        let single_number = find_single_number(nums);
        assert_eq!(2, single_number);
    }
}
