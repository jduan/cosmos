/// You can use the ? operator on Options.  If x is an Option, then evaluating x? will
/// 1. return the underlying value if x is Some,
/// 2. otherwise it will terminate whatever function is being executed and return None.
pub fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

/// You can chain many ?s together to make your code much more readable.
pub struct Person {
    pub job: Option<Job>,
}

pub struct Job {
    pub phone_number: Option<PhoneNumber>,
}

pub struct PhoneNumber {
    pub area_code: Option<u8>,
    pub number: u32,
}

impl Person {
    pub fn work_phone_area_code(self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        self.job?.phone_number?.area_code
    }
}

/// Combinators:
/// 1. map: maps Option to Option (similar to map a function over a collection)
/// 2. and_then: map a function over an Option but the function itself returns an Option too

/// match is a valid method for handling Options. However, you may eventually find heavy
/// usage tedious, especially with operations only valid with an input. In these cases,
/// combinators can be used to manage control flow in a modular fashion.
///
/// Option has a built in method called map(), a combinator for the simple mapping of
/// Some -> Some and None -> None. Multiple map() calls can be chained together for even
/// more flexibility.
#[derive(Debug)]
pub enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
pub struct Peeled(Food);
#[derive(Debug)]
pub struct Chopped(Food);
#[derive(Debug)]
pub struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`. Otherwise, return the peeled food.
pub fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Peeling food. If there isn't any, then return `None`. Otherwise, return the peeled food.
pub fn chop(food: Option<Peeled>) -> Option<Chopped> {
    match food {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Peeling food. If there isn't any, then return `None`. Otherwise, return the peeled food.
pub fn cook(food: Option<Chopped>) -> Option<Cooked> {
    match food {
        Some(Chopped(food)) => Some(Cooked(food)),
        None => None,
    }
}

// A function to peel, chop, and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
pub fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|food| Peeled(food))
        .map(|Peeled(food)| Chopped(food))
        .map(|Chopped(food)| Cooked(food))
}

pub fn eat(food: Option<Cooked>) {
    match food {
        Some(Cooked(food)) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

/// map() was described as a chainable way to simplify match statements. However,
/// using map() on a function that returns an Option<T> results in the nested
/// Option<Option<T>>. Chaining multiple calls together can then become confusing.
/// That's where another combinator called and_then(), known in some languages as
/// flatmap, comes in.
pub fn get_shortest(_names: Vec<&str>) -> Option<&str> {
    unimplemented!()
}

pub struct JSON {}
pub struct User {}

pub fn find_user_by_name(_name: &str) -> Option<JSON> {
    unimplemented!()
}

pub fn json_to_user(_json: JSON) -> Option<User> {
    unimplemented!()
}

pub fn get_user_with_shortest_name(names: Vec<&str>) -> Option<User> {
    get_shortest(names)
        .and_then(|shortest| find_user_by_name(shortest))
        .and_then(|user| json_to_user(user))
}

// This is equivalent to the function above but much more verbose!
pub fn get_user_with_shortest_name2(names: Vec<&str>) -> Option<User> {
    match get_shortest(names) {
        None => None,
        Some(name) => match find_user_by_name(name) {
            None => None,
            Some(json) => match json_to_user(json) {
                None => None,
                Some(user) => Some(user),
            },
        },
    }
}

pub fn run() {
    let _some_number = Some(5);
    let _some_string = Some("a string");
    // the type is needed because Rust can't infer the type for None
    let _absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    match six {
        Some(i) => println!("five plus one is {}", i),
        None => println!("this should never happen"),
    }

    placeholder(3);
    placeholder(7);
    placeholder(30);
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

pub fn placeholder(num: u8) {
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

    #[test]
    fn test_map_combinators() {
        let apple = Food::Apple;
        let carrot = Food::Carrot;
        let potato = None;

        let cooked_apple = cook(chop(peel(Some(apple))));
        let cooked_carrot = cook(chop(peel(Some(carrot))));
        let cooked_potato = cook(chop(peel(potato)));

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
}
