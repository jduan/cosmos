/// You can use the ? operator on Options.  If x is an Option, then evaluating x? will
/// 1. return the underlying value if x is Some,
/// 2. otherwise it will terminate whatever function is being executed and return None.
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

/// You can chain many ?s together to make your code much more readable.
struct Person {
    job: Option<Job>,
}

struct Job {
    phone_number: Option<PhoneNumber>,
}

struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        self.job?.phone_number?.area_code
    }
}

pub fn run() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // the type is needed because Rust can't infer the type for None
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(i) => println!("five plus one is {}", i),
        None => println!("this should never happen"),
    }

    placeholder(3);
    placeholder(7);
    placeholder(30);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn placeholder(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // use _ to match everything else!
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_birthday() {
        let age = None;
        assert_eq!(None, next_birthday(age));

        let age = Some(99u8);
        assert_eq!(
            Some(String::from("Next year I will be 99")),
            next_birthday(age)
        );
    }

    #[test]
    fn test_chain_question_mark_operator() {
        let person = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 1234567,
                }),
            }),
        };

        assert_eq!(61, person.work_phone_area_code().unwrap());
    }
}
