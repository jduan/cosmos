#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};

    // Most of the control flows in C are statements. In Rust, they are all expressions.
    #[test]
    fn almost_everything_is_an_expression() {
        let temperature = 100;
        let status = if temperature > 72 { "hot" } else { "cool" };
        assert_eq!(status, "hot");
    }

    #[test]
    fn blocks_are_expressions() {
        let author = Some("David");
        let display_name = match author {
            Some(name) => format!("Hi {}", name),
            None => {
                // the value of this block is its last expression
                // get the name from another source, such as over the network
                "Hi John".to_string()
            }
        };

        assert_eq!(display_name, "Hi David");
    }

    // It’s an error to use a variable before it’s initialized. (This is closely related to the
    // error of using a value after it’s been moved. Rust really wants you to use values only while
    // they exist!)
    #[test]
    fn variables_must_be_initialized_before_use() {
        let name;
        let flag = true; // in reality, the flag can be dynamic
        if flag {
            name = "John";
        } else {
            name = "David";
            // may do some other things after assigning name
        }

        // now that "name" has been initialized, you can use it
        assert_eq!(name, "John");
    }

    #[test]
    fn redeclare_variables() -> io::Result<()> {
        // create the file first
        let path = "/tmp/temp_rust_file";
        let mut output = File::create(path)?;
        write!(output, "Rust is fun!")?;

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // The line on the right hand side is the orignal line and its type is
            // Result<String, io:Error>.
            // The line on the left hand side is the new line and its type is String.
            let line = line?;
            println!("read line: {}", line);
        }

        Ok(())
    }

    #[test]
    fn nested_functions() {
        // You can define a function inside a block and its scope is only this block.
        // However, a nested fn can't access local variables or arguments that happen to be in
        // scope. For that, you need "closures". In other words, nested "functions" aren't
        // closures.
        fn compare(m: i32, n: i32) -> &'static str {
            if m > n {
                "greater than"
            } else if m == n {
                "equal to"
            } else {
                "less than"
            }
        }

        assert_eq!("greater than", compare(10, 3));
    }

    // "if let" expression is a shorthand for:
    // match expr {
    //      pattern => { block1 }
    //      _ => { block2 }
    // }
    #[test]
    fn if_let_expression() {
        let cookie = Some(2);
        if let Some(n) = cookie {
            assert_eq!(n, 2);
        }

        let result: Result<i32, &str> = Err("api failed");
        if let Err(msg) = result {
            assert_eq!(msg, "api failed");
        } else {
            panic!("This should never happen.");
        }
    }

    #[test]
    fn four_kinds_of_loops() {
        // while, while let, loop, and for
        let mut n = 0;
        while n < 3 {
            println!("Hi {}", n);
            n += 1;
        }

        // "while let" looks like this:
        // while let pattern = expr {
        //      block
        // }

        let mut n = 0;
        loop {
            println!("Hi {}", n);
            n += 1;
            if n == 3 {
                break;
            }
        }

        // 0..3 is the same as:
        // std::ops::Range { start: 0, end: 20 }
        for n in 0..3 {
            println!("Hi {}", n);
        }

        let names = ["hello", "world"];
        // A "for" loop over a value consumes the value. You can't use it anymore after the loop.
        // Hence here we use a ref "&names" so we can still access "names" after the loop.
        for name in &names {
            println!("name is {}", name);
        }
        assert_eq!(2, names.len());

        // A loop can be labeled with a lifetime and can be used for "break".
        let cities = vec![
            vec!["San Francisco", "Oakland"],
            vec!["Beijing", "Shanghai"],
        ];
        let mut count = 0;
        'outer: for names in cities {
            for name in names {
                count += 1;
                if name == "Beijing" {
                    println!("Going to stop here.");
                    break 'outer;
                }
            }
        }
        assert_eq!(3, count);
    }

    #[test]
    fn special_type_bang() {
        // In Rust, some expressions don't have a normal type. Expressions that don't finish
        // normally are assigned the special type ! and they're exempt from the rules about types
        // having to match. Such expressions are "break, return, loop, panic!, std::process::exit"
        // etc.
        // Note that you can define your functions to return ! as well. An example is a function
        // that starts a web server and that returns.
        let m = 5;
        let n = 3;
        // this line won't compile
        // let name = if m > n { "John" } else { 10 };

        // This compiles just fine.
        let name = if m > n {
            "John"
        } else {
            panic!("This should never happen!");
        };
        assert_eq!("John", name);
    }

    #[test]
    fn type_cast() {
        let x = 17; // x is type i32
        let index = x as usize; // "as" the type casting keyword
        assert_eq!(index, 17usize);

        // Rust also does automatic conversions, such as:
        // Values of type "&String" auto convert to "&str"
        // Values of type "&Vec[i32]" auto convert to "&[i32]"
        // Values of type "&Box<Chessboard>" auto convert to "&Chessboard"
        //
        // These are called "deref coercions", because they apply to types that implement the
        // "Deref" trait.
    }

    #[test]
    fn closures() {
        // Rust infers the argument types and return type.
        let is_even = |x| x % 2 == 0;
        assert_eq!(is_even(4), true);

        // You can also write out the types explicitly. When you do that, the body of the closure
        // must be a block.
        let is_even = |x: u64| -> bool { x % 2 == 0 };
        assert_eq!(is_even(5), false);
    }
}
