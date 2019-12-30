use std::str::Chars;

pub struct Lexer<'a> {
    //    position: usize,      // current position in input
    //    read_position: usize, // next position in input, used for peaking at the next char
    iter: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            //            position: 0,
            //            read_position: 1,
            iter: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(ch) = self.iter.next() {
            match ch {
                ',' => Token::Delimiter(Delimiter::Comma),
                '{' => Token::Delimiter(Delimiter::LeftBrace),
                '(' => Token::Delimiter(Delimiter::LeftParen),
                '}' => Token::Delimiter(Delimiter::RightBrace),
                ')' => Token::Delimiter(Delimiter::RightParen),
                ';' => Token::Delimiter(Delimiter::Semicolon),
                '=' => Token::Operator(Operator::Assignment),
                '+' => Token::Operator(Operator::PlusSign),
                // TODO: handle other kinds of tokens, such as identifiers, keywords etc.
                _ => Token::SpecialToken(SpecialToken::Illegal),
            }
        } else {
            Token::SpecialToken(SpecialToken::EOF)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    SpecialToken(SpecialToken),
    Identifier(Identifier),
    Literal(Literal),
    Operator(Operator),
    Delimiter(Delimiter),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpecialToken {
    EOF,
    Illegal,
}

pub type Identifier = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Integer(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Assignment,
    PlusSign,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Comma,
    LeftBrace,
    LeftParen,
    RightBrace,
    RightParen,
    Semicolon,
}

pub enum Keyword {
    Function,
    Let,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_char() {
        let mut lexer = Lexer::new("=+(){},;!");
        assert_eq!(Token::Operator(Operator::Assignment), lexer.next_token());
        assert_eq!(Token::Operator(Operator::PlusSign), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::LeftParen), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::RightParen), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::LeftBrace), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::RightBrace), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::Comma), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::Semicolon), lexer.next_token());
        assert_eq!(
            Token::SpecialToken(SpecialToken::Illegal),
            lexer.next_token()
        );
        assert_eq!(Token::SpecialToken(SpecialToken::EOF), lexer.next_token());
    }
}
