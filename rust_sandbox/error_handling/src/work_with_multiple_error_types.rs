/// Composing Option and Result

#[allow(dead_code)]
fn double_arg(argv: Vec<&str>) -> Result<i32, String> {
    argv.first()
        .ok_or_else(|| "Expected at least one argument".to_owned())
        .and_then(|str| str.parse::<i32>().map_err(|err| err.to_string()))
        .map(|i| i * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_arg() {
        let argv = vec!["3", "hello"];
        assert_eq!(6, double_arg(argv).unwrap());
    }

    #[test]
    fn test_double_arg2() {
        let argv = vec!["world", "hello"];
        assert_eq!(
            "invalid digit found in string",
            double_arg(argv).err().unwrap()
        );
    }
}
