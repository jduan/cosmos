use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

// A simple implementation of `% cat path`
pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// A simple implementation of `% echo s > path`
pub fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

// A simple implementation of `% touch path` (ignores existing files)
pub fn touch(path: &Path) -> io::Result<()> {
    OpenOptions::new().create(true).write(true).open(path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix;
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

    #[test]
    fn various_file_system_operations() {
        println!("`mkdir a`");
        // Create a directory, returns `io::Result<()>`
        match fs::create_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {}
        }

        println!("`echo hello > a/b.txt`");
        // The previous match can be simplified using the `unwrap_or_else` method
        echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`mkdir -p a/c/d`");
        // Recursively create a directory, returns `io::Result<()>`
        fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`touch a/c/e.txt`");
        touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`ln -s ../b.txt a/c/b.txt`");
        // Create a symbolic link, returns `io::Result<()>`
        if cfg!(target_family = "unix") {
            unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }

        println!("`cat a/c/b.txt`");
        match cat(&Path::new("a/c/b.txt")) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
        }

        println!("`ls a`");
        // Read the contents of a directory, returns `io::Result<Vec<Path>>`
        match fs::read_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    println!("> {:?}", path.unwrap().path());
                }
            }
        }

        println!("`rm a/c/e.txt`");
        // Remove a file, returns `io::Result<()>`
        fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`rmdir a/c/d`");
        // Remove an empty directory, returns `io::Result<()>`
        fs::remove_dir("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`rm -rf a`");
        fs::remove_dir_all("a").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
}
