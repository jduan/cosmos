pub fn run() {
    let some_u8_value = Some(3);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // the code below behaves exactly the same but it's more concise.

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("None");
    }

    match_number(None);
}

pub fn match_number(num: Option<u8>) -> &'static str {
    if let Some(3) = num {
        "three"
    } else {
        "None"
    }
}

pub enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn match_enum(foo: Foo) -> &'static str {
    if let Foo::Bar = foo {
        "foobar"
    } else if let Foo::Baz = foo {
        "foobaz"
    } else {
        "qux"
    }
}

#[cfg(test)]
mod tests {
    use crate::if_let::{Foo, match_enum, match_number};

    #[test]
    fn test_match_number() {
        assert_eq!(match_number(Some(3)), "three");
        assert_eq!(match_number(None), "None");
    }

    #[test]
    fn test_match_enum() {
        assert_eq!(match_enum(Foo::Bar), "foobar");
        assert_eq!(match_enum(Foo::Baz), "foobaz");
        assert_eq!(match_enum(Foo::Qux(8)), "qux");
    }
}
