mod arrays;
mod slice_type;
mod smart_pointers;
mod vectors;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

fn might_panic(value: i32) -> i32 {
    if value < 1 || value > 100 {
        panic!("input value must be between 1 and 100");
    } else {
        value + 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        might_panic(102);
    }

    #[test]
    #[should_panic(expected = "input value must be between 1 and 100")]
    fn less_than_1() {
        might_panic(0);
    }

    #[test]
    fn less_than_100() {
        assert_eq!(199, might_panic(99), "should be equal to 199");
    }

    // You can make tests return Result<T, E>
    // This enables you to use the ? operator in the body of tests, which can be
    // a convenient way to write tests taht should fail if any operation within
    // them returns an "Err" variant.
    #[test]
    fn it_works() -> Result<(), String> {
        if add_two(3) == 5 {
            Ok(())
        } else {
            Err(String::from("two plus three doesn't equal to five!"))
        }
    }
}
