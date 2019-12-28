type InputOutput<'a> = (&'a str, ());

/// This function returns a parser function that matches the input string with
/// an expected string.
fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<InputOutput, &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_match_literal() {
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
        assert_eq!(Ok((" Hello Robert!", ())), parse_joe("Hello Joe! Hello Robert!"));
        assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
    }
}
