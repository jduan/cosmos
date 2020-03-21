use std::thread;

/// In this example, we will calculate the sum of all digits in a block of numbers. We will
/// do this by parcelling out chunks of the block into different threads. Each thread will
/// sum its tiny block of digits, and subsequently we will sum the intermediate sums produced
/// by each thread.
pub fn sum_all_digits(digits: &str) -> u32 {
    let mut children = vec![];
    for (_idx, data) in digits.split_whitespace().enumerate() {
        // We need to make a copy of the chunk here, otherwise the chunk won't live
        // as long as the thread itself. With the copy, the data_copy value will be owned
        // by the thread and everything will be good.
        let data_copy = String::from(data);
        let thread = thread::spawn(move || -> u32 { sum_digits(&data_copy) });
        children.push(thread);
    }

    let mut sum = 0;
    for child in children {
        sum += child.join().unwrap();
    }

    sum
}

pub fn sum_digits(digits: &str) -> u32 {
    digits
        .chars()
        .map(|ch| ch.to_digit(10).expect("Expect a digit!"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digits() {
        assert_eq!(20, sum_digits("341345"));
    }

    #[test]
    fn test_sum_all_digits() {
        assert_eq!(45, sum_all_digits("123 456 789"));
        assert_eq!(69, sum_all_digits("123 456 789 888"));
    }
}
