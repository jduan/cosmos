pub fn run() {
    println!("Penny is worth {}", value_in_cents(Coin::Penny));
    println!("Dime is worth {}", value_in_cents(Coin::Dime));
    println!(
        "Quarter(California) is worth {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
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

fn get_number_value(n: u32) -> &'static str {
    match n {
        1 => "One",
        2 | 3 | 5 | 7 | 11 => "Prime",
        13...19 => "Teen",
        _ => "Aren't special",
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern_matching::get_number_value;

    #[test]
    fn test_get_number_value() {
        assert_eq!("One", get_number_value(1));
        assert_eq!("Prime", get_number_value(3));
        assert_eq!("Teen", get_number_value(13));
        assert_eq!("Teen", get_number_value(19));
        assert_eq!("Aren't special", get_number_value(190));
    }
}
