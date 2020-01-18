pub fn run() {
    let number = 6;

    if number % 4 == 0 {
        println!("Your number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Your number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Your number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // if is an expression so you can assign it to a var
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is {}", number);

    loops();
}

pub fn loops() {
    // loop {
    //     // this loops forever until you call "break"
    //     println!("again!");
    // }

    let mut number = 3;
    while number != 0 {
        println!("number is {}", number);
        number -= 1;
    }
    println!("lift off!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the element is {}", element);
    }

    // (1..4) is a range
    for number in (1..4).rev() {
        println!("number is {}", number);
    }
}

pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    // From 1 to 15
    for i in 1..=n {
        if i % 15 == 0 {
            words.push(String::from("fizzbuzz"));
        } else if i % 3 == 0 {
            words.push(String::from("fizz"));
        } else if i % 5 == 0 {
            words.push(String::from("buzz"));
        } else {
            words.push(format!("{}", i));
        }
    }

    words
}

#[cfg(test)]
mod tests {
    use crate::control_flow::fizzbuzz;

    #[test]
    fn test_loops() {
        let words = fizzbuzz(15);
        let expected_words = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz",
        ];
        assert_eq!(expected_words, words);
    }
}
