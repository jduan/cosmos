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

fn match_number(num: Option<u8>) {
    if let Some(3) = num {
        println!("three");
    } else {
        println!("None");
    }
}
