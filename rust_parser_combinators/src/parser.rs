/// We're going to write a parser for a simplified version of XML. It looks like this:
///
/// ```
/// <parent-element>
///   <single-element attribute="value" />
/// </parent-element>
/// ```

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

/// Implement the Parser trait for any function that matches the signature of a parser.
impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

/// This function returns a parser function that matches the input string with
/// an expected string.
fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

/// Expects one or more whitespaces.
fn one_or_more_whitespaces<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

/// Expects one or more whitespaces.
fn zero_or_more_whitespaces<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

/// Matches one or more things.
fn one_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    move |mut input| {
        let mut result: Vec<R> = vec![];
        match parser.parse(input) {
            Ok((next_input, r)) => {
                result.push(r);
                input = next_input;
                while let Ok((next_input, r)) = parser.parse(input) {
                    result.push(r);
                    input = next_input;
                }
            }
            Err(err) => return Err(err),
        }

        Ok((input, result))
    }
}

/// Matches zero or more things.
fn zero_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    move |mut input| {
        let mut result: Vec<R> = vec![];
        while let Ok((next_input, r)) = parser.parse(input) {
            result.push(r);
            input = next_input;
        }

        Ok((input, result))
    }
}

/// Matches any character.
fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(ch) => Ok((&input[ch.len_utf8()..], ch)),
        None => Err(input),
    }
}

fn predicate<'a, P, R, F>(parser: P, pred: F) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
    F: Fn(&R) -> bool,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => {
            if pred(&result) {
                Ok((next_input, result))
            } else {
                Err(input)
            }
        }
        Err(err) => Err(err),
    }
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    predicate(any_char, |ch| ch.is_whitespace())
}

/// Parse the next identifier.
/// An identifier starts with one alphabetical character and is followed by zero or more
/// of either an alphabetical character, a number, or a dash.
fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

/// This is a parser combiner. It combines two parsers, P1 and P2, and return another parser
/// that applies both P1 and P2 and returns a pair of (R1, R2).
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(final_input, result2)| (final_input, (result1, result2)))
        })
    }
}

/// This is a combinator that changes the type of the result of a parser by applying another function.
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

/// This is a parser combiner. It combines two parsers, P1 and P2, but only returns the
/// result of the first parser, discarding the result of the second parser.
fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(r1, _r2)| r1)
}

/// This is a parser combiner. It combines two parsers, P1 and P2, but only returns the
/// result of the second parser, discarding the result of the first parser.
fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_r1, r2)| r2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_match_literal() {
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe.parse("Hello Joe! Hello Robert!")
        );
        assert_eq!(Err("Hello Mike!"), parse_joe.parse("Hello Mike!"));
    }

    #[test]
    fn test_one_or_more_whitespaces() {
        assert_eq!(
            Ok(("hello", vec![' ',])),
            one_or_more_whitespaces().parse(" hello")
        );
        assert_eq!(
            Ok(("hello", vec![' ', '\t'])),
            one_or_more_whitespaces().parse(" \thello")
        );
        assert_eq!(
            Ok(("hello", vec![' ', '\t', ' '])),
            one_or_more_whitespaces().parse(" \t hello")
        );
    }

    #[test]
    fn test_zero_or_more_whitespaces() {
        assert_eq!(
            Ok(("hello", vec![])),
            zero_or_more_whitespaces().parse("hello")
        );
        assert_eq!(
            Ok(("hello", vec![' ',])),
            zero_or_more_whitespaces().parse(" hello")
        );
        assert_eq!(
            Ok(("hello", vec![' ', '\t'])),
            zero_or_more_whitespaces().parse(" \thello")
        );
        assert_eq!(
            Ok(("hello", vec![' ', '\t', ' '])),
            zero_or_more_whitespaces().parse(" \t hello")
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            Ok(("", "i-am-an-identifier".to_string())),
            identifier("i-am-an-identifier")
        );
        assert_eq!(
            Ok((" entirely an identifier", "not".to_string())),
            identifier("not entirely an identifier")
        );
        assert_eq!(
            Err("!not at all an identifier"),
            identifier("!not at all an identifier")
        );
    }

    #[test]
    fn test_pair() {
        let tag_opener = pair(match_literal("<"), identifier);
        assert_eq!(
            Ok(("/>", ((), "my-first-element".to_string()))),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn test_map() {
        let tag_opener = map(pair(match_literal("<"), identifier), |(_left, right)| right);
        assert_eq!(
            Ok(("/>", "my-first-element".to_string())),
            tag_opener.parse("<my-first-element/>")
        );
    }

    #[test]
    fn test_left() {
        let tag_opener = left(match_literal("<"), identifier);
        assert_eq!(Ok(("/>", ())), tag_opener.parse("<my-first-element/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn test_right() {
        let tag_opener = right(match_literal("<"), identifier);
        assert_eq!(
            Ok(("/>", "my-first-element".to_string())),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn test_one_or_more() {
        let parser = one_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
        assert_eq!(Err("ahah"), parser.parse("ahah"));
        assert_eq!(Err(""), parser.parse(""));
    }

    #[test]
    fn zero_or_more_combinator() {
        let parser = zero_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
        assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
    }

    #[test]
    fn test_any_char() {
        assert_eq!(Ok(("ello", 'H')), any_char("Hello"));
        assert_eq!(Err(""), any_char(""));
    }

    #[test]
    fn test_predicate() {
        let parser = predicate(any_char, |ch| ch.is_whitespace());
        assert_eq!(Ok(("hello", ' ')), parser.parse(" hello"));
        assert_eq!(Err("!hello"), parser.parse("!hello"));
    }

    #[test]
    fn test_whitespace_char() {
        let parser = whitespace_char();
        assert_eq!(Ok(("hello", ' ')), parser.parse(" hello"));
        assert_eq!(Ok(("hello", '\t')), parser.parse("\thello"));
        assert_eq!(Err("!hello"), parser.parse("!hello"));
    }
}
