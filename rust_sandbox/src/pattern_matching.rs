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
