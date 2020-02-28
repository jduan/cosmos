pub fn sum(n: u32) -> u32 {
    let mut total = 0;
    let mut count = Some(0);

    // "while let" doesn't have an "else" branch. When the pattern doesn't match, the loop ends.
    while let Some(i) = count {
        if i > n {
            count = None;
        } else {
            total += i;
            count = Some(i + 1);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::while_let::sum;

    #[test]
    fn test_sum() {
        assert_eq!(sum(10), 55);
    }
}
