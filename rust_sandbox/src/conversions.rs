use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

/// Rust addresses conversion between types by the use of traits. The generic conversions will use
/// the From and Into traits. However there are more specific ones for the more common cases, in
/// particular when converting to and from Strings.

/// The From and Into traits are inherently linked, and this is actually part of its implementation.
/// If you are able to convert type A from type B, then it should be easy to believe that we should
/// be able to convert type B to type A.

#[derive(Debug)]
struct Number {
    value: i32,
}

/// The "From" trait allows for a type to define how to create itself from another type.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/// "TryFrom" and "TryInto" are generic traits for converting between types. Unlike From/Into,
/// the TryFrom/TryInto traits are used for fallible conversions, and as such, return "Result"s.
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

/// ToString and FromStr traits.
///
/// To convert any type to a String is as simple as implementing the ToString trait for the type.
/// Rather than doing so directly, you should implement the fmt::Display trait which automagically
/// provides ToString and also allows printing the type.
struct Circle {
    radius: u32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Circle of radius {}", self.radius)
    }
}

/// The FromStr trait is for converting a &str into a type T.
impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Circle of radius (\d*)$").unwrap();
        let cap = re.captures(s);
        match cap {
            Some(caps) => {
                let radius = caps.get(1).unwrap().as_str().parse().unwrap();
                Ok(Circle { radius })
            }
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_from() {
        let my_str = "hello";
        let my_string = String::from(my_str);
        assert_eq!(5, my_string.len());
    }

    #[test]
    fn test_number() {
        let num = Number::from(30);
        assert_eq!(30, num.value);

        let i = 5;
        // The "Into" trait is simply the reciprocal of the "From" trait. If you have implemented
        // the "From" trait for a type, you get the "Into" implementation for free.
        // The only thing is you need to provide the "target type" to convert into because the
        // compiler is unable to determine this most of the time.
        let num2: Number = i.into();
        assert_eq!(5, num2.value);
    }

    #[test]
    fn test_try_from() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(9), Err(()));

        // You get "TryInto" for free!
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 9i32.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_string() {
        let circle = Circle { radius: 6 };
        assert_eq!(circle.to_string(), String::from("Circle of radius 6"));
    }

    #[test]
    fn test_from_string() {
        let circle = Circle::from_str("Circle of radius 6").unwrap();
        assert_eq!(6, circle.radius);

        let circle = Circle::from_str("This is not a circle");
        assert!(circle.is_err());

        // convert a &str to an integer
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        assert_eq!(15, parsed + turbo_parsed);
    }
}
