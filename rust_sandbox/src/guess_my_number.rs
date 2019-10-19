use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // shadows the previous "guess" var. This feature is often used in situations
        // in which you want to convert a value from one type to another type.
        // Shadowling lets us reuse the same var name rather than forcing us to
        // create two unique vars.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(line) => {
                println!("Can't parse input into a number. Error: {}", line);
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
