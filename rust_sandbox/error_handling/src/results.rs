use std::num::ParseIntError;

#[allow(dead_code)]
fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

#[allow(dead_code)]
fn double_number2(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|i| i * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_number() {
        assert_eq!(6, double_number("3"));
    }

    #[test]
    #[should_panic(expected = "ParseIntError")]
    fn test_double_number2() {
        double_number("hello");
    }

    #[test]
    fn test_double_number3() {
        assert_eq!(6, double_number2("3").unwrap());
    }
}
