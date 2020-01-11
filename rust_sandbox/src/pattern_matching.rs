use crate::pattern_matching::Color::*;

pub fn run() {
    println!("Penny is worth {}", value_in_cents(Coin::Penny));
    println!("Dime is worth {}", value_in_cents(Coin::Dime));
    println!(
        "Quarter(California) is worth {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    California,
    Washington,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

pub fn get_number_value(n: u32) -> &'static str {
    match n {
        1 => "One",
        2 | 3 | 5 | 7 | 11 => "Prime",
        13...19 => "Teen",
        _ => "Aren't special",
    }
}

pub fn destruct_tuples(pair: (i32, i32)) -> String {
    match pair {
        (0, y) => format!("First is 0 and y is {}", y),
        (x, 0) => format!("x is {} and last is 0", x),
        _ => String::from("It doesn't matter what they are"),
    }
}

pub enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
}

pub fn destruct_enums(color: Color) -> String {
    match color {
        Red => String::from("Red"),
        Blue => String::from("Blue"),
        Green => String::from("Green"),
        RGB(r, g, b) => format!("Red: {}, Green: {}, Blue: {}", r, g, b),
    }
}

pub struct Foo {
    x: (u32, u32),
    y: u32,
}

pub fn destruct_structs(foo: Foo) -> String {
    match foo {
        Foo { x: (1, b), y } => format!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: x } => format!("y is 2, x = {:?}", x),
        // Use ".." to ignore some fields
        Foo { y, .. } => format!("y = {}, and we don't care about x", y),
    }
}

// A match guard can be added to filter the arm.
pub fn use_guard(pair: (i32, i32)) -> &'static str {
    match pair {
        (x, y) if x == y => "These are twins",
        (x, y) if x + y == 0 => "Antimatter, kaboom!",
        (x, _) if x % 2 == 1 => "The first one is odd",
        _ => "No correlation",
    }
}

// Binding: use @ sigil to bind values to names
pub fn report_age(age: u32) -> String {
    match age {
        0 => String::from("I'm not born yet I guess"),
        n @ 1..=12 => format!("I'm a child of age {}", n),
        n @ 13..=19 => format!("I'm a teen of age {}", n),
        n => format!("I'm an old person of age {}", n),
    }
}

pub fn use_binding(number: Option<u32>) -> String {
    match number {
        Some(n @ 42) => format!("The answer is {}", n),
        Some(n) => format!("Not interesting {}", n),
        _ => String::from("I'm None"),
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern_matching::{
        Color, destruct_enums, destruct_structs, destruct_tuples, Foo,
        get_number_value, report_age, use_binding, use_guard,
    };

    #[test]
    fn test_get_number_value() {
        assert_eq!("One", get_number_value(1));
        assert_eq!("Prime", get_number_value(3));
        assert_eq!("Teen", get_number_value(13));
        assert_eq!("Teen", get_number_value(19));
        assert_eq!("Aren't special", get_number_value(190));
    }

    #[test]
    fn test_destruct_tuples() {
        assert_eq!("First is 0 and y is 3", destruct_tuples((0, 3)));
        assert_eq!("x is 3 and last is 0", destruct_tuples((3, 0)));
        assert_eq!("It doesn't matter what they are", destruct_tuples((30, 10)));
    }

    #[test]
    fn test_destruct_enums() {
        assert_eq!("Red", destruct_enums(Color::Red));
        assert_eq!("Green", destruct_enums(Color::Green));
        assert_eq!("Blue", destruct_enums(Color::Blue));
        assert_eq!(
            "Red: 100, Green: 200, Blue: 255",
            destruct_enums(Color::RGB(100, 200, 255))
        );
    }

    #[test]
    fn test_destruct_structs() {
        assert_eq!(
            String::from("First of x is 1, b = 2, y = 3"),
            destruct_structs(Foo { x: (1, 2), y: 3 })
        );
        assert_eq!(
            String::from("y is 2, x = (10, 2)"),
            destruct_structs(Foo { x: (10, 2), y: 2 })
        );
        assert_eq!(
            String::from("y = 5, and we don't care about x"),
            destruct_structs(Foo { x: (10, 2), y: 5 })
        );
    }

    #[test]
    fn test_use_guard() {
        assert_eq!(use_guard((1, 1)), "These are twins");
        assert_eq!(use_guard((1, -1)), "Antimatter, kaboom!");
        assert_eq!(use_guard((3, 4)), "The first one is odd");
        assert_eq!(use_guard((6, 7)), "No correlation");
    }

    #[test]
    fn test_binding() {
        assert_eq!(report_age(0), String::from("I'm not born yet I guess"));
        assert_eq!(report_age(3), String::from("I'm a child of age 3"));
        assert_eq!(report_age(13), String::from("I'm a teen of age 13"));
        assert_eq!(report_age(30), String::from("I'm an old person of age 30"));
    }

    #[test]
    fn test_binding2() {
        assert_eq!(use_binding(Some(42)), String::from("The answer is 42"));
        assert_eq!(use_binding(Some(21)), String::from("Not interesting 21"));
        assert_eq!(use_binding(None), String::from("I'm None"));
    }
}
