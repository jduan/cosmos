#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::path::Path;

    static FILEPATH: &str = "/tmp/jkfdsjkfdj";
    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

    #[test]
    fn create_file() {
        let path = Path::new(FILEPATH);
        println!("About to create a file {}", path.display());

        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create file {}: {}", path.display(), why),
            Ok(file) => file,
        };

        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to file {}: {}", path.display(), why),
            Ok(_) => println!("successfully wrote to file {}", path.display()),
        };
    }

    #[test]
    fn read_file() {
        create_file();

        let mut file = match File::open(FILEPATH) {
            Err(why) => panic!("couldn't open file {}: {}", FILEPATH, why),
            Ok(file) => file,
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read file {}: {}", FILEPATH, why),
            Ok(_) => assert_eq!(447, content.len()),
        };
    }

    #[test]
    fn read_lines() -> Result<(), std::io::Error> {
        create_file();

        let file = File::open(FILEPATH)?;
        let lines = BufReader::new(file).lines();
        let mut count = 0;
        for line in lines {
            count += 1;
            let line = line?;
            println!("{}", line);
        }

        assert_eq!(6, count);
        Ok(())
    }
}
