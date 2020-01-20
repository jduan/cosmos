/// https://doc.rust-lang.org/stable/std/io/
///
/// # Traits:
/// Read        byte-oriented
/// Write       both byte-oriented and utf8 text
/// BufRead     uses an internal buffer to provide a number of other ways to read
/// Seek        lets you control where the next byte is coming from
///
/// # Structs:
/// BufReader   works with the BufRead trait to add extra methods to any reader
/// BufWriter   doesn't add any new ways of writing; it just buffers every call to write
/// Stdin
/// Stdout
/// File
/// TcpStream
/// Chain       Adaptor to chain together two readers
/// Cursor      wraps an in-memory buffer and provides it with a Seek implementation
/// Lines       An iterator over the lines of an instance of BufRead
/// Vec<u8>
/// ... and more
///
/// Note that a File isn't automatically buffered in Rust. File implements Read but not BufRead.
/// To get a buffered reader, do "BufReader::new(file)"
///
/// In most languages, files are buffered by default. If you want unbuffered input or output, you
/// have to figure out how to turn buffering off. In Rust, File and BufReader are two separate
/// library features, because sometimes you want files without buffering, and sometimes you want
/// buffering without files (for example, you may want to buffer input from the network).
///
/// # std::io::Read has several methods for reading data
///
/// reader.read(&mut buffer) -> io::Result<u64>
/// reader.read_to_end(&mut byte_vec)
/// reader.read_to_string(&mut string)
/// reader.read_exact(&mut buffer)
/// reader.bytes()      returns an iterator over bytes
/// reader.chars()      returns an iterator over characters
/// reader.chain(reader2)
/// reader.take(n)      returns a new reader that reads from the same source but is limited
///                     to n bytes of input
///
/// # std::io::BufRead
///
/// reader.read_line(&mut line)     reads a line of utf8 text (\n is kept)
/// reader.lines()                  returns an iterator of the lines of the input (\n is not kept)
/// reader.read_until(stop_byte, &mut byte_vec)     similar to read_line() but is byte-oriented
/// reader.split(stop_byte)      similar to lines() but is byte-oriented
///
/// # std::io::Write
///
/// writer.write(&buf)      returns io::Result<usize>
/// writer.write_all(&buf)  writes all the bytes in the slice buf
/// writer.flush()          flushes any buffered data to the underlying stream
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// Since there are standard traits for readers and writers, it's quite common to write
/// generic code that works across a variety of input or output channels. For example,
/// here's function that copies all bytes from any reader to any writer.
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut written = 0u64;
    let mut buffer = [0; 1024];
    loop {
        let len = match reader.read(&mut buffer) {
            Ok(0) => return Ok(written),
            Ok(n) => n,
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        };

        writer.write_all(&buffer[..len])?;
        written += len as u64;
    }
}

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

/// This function works with any reader that implements BufRead, such as stdin.lock, BufReader, etc
pub fn grep<R>(target: &str, reader: R) -> io::Result<Vec<String>>
where
    R: BufRead,
{
    let mut matched_lines = vec![];
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            matched_lines.push(line);
        }
    }

    Ok(matched_lines)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Read, SeekFrom, Write};
    use std::os::unix;
    use std::path::Path;

    use super::*;
    use tempfile::NamedTempFile;

    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
    fn create_file() -> io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        file.write_all(LOREM_IPSUM.as_bytes())?;

        Ok(file)
    }

    /// To open a file, you usually do "File::open(filename)" or "File::create(filename)".
    /// When neither fits the bill, you can use OpenOptions to specify the exact desired behavior.
    #[allow(dead_code)]
    fn open_options() -> io::Result<()> {
        let _log = OpenOptions::new()
            .append(true) // if file exists, add to the end
            .open("/tmp/server.log")?;
        let _file = OpenOptions::new()
            .write(true)
            .create_new(true) // fail if file exits
            .open("/tmp/new_file.txt")?;

        Ok(())
    }

    #[test]
    fn test_copy() -> io::Result<()> {
        let tempfile = create_file()?;
        let mut file = File::open(tempfile)?;
        let mut output: Vec<u8> = vec![];
        let written = copy(&mut file, &mut output)?;
        assert_eq!(447, written);

        Ok(())
    }

    #[test]
    fn read_file() {
        let file = create_file().unwrap();
        let path = file.path();

        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open file {:?}: {}", path, why),
            Ok(file) => file,
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read file {:?}: {}", path, why),
            Ok(n) => assert_eq!(447, n),
        };
    }

    #[test]
    fn test_grep() -> io::Result<()> {
        let file = create_file()?;
        let reader = BufReader::new(File::open(file.path())?);
        let matches = grep("ad", reader)?;
        assert_eq!(
            vec![
                "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod",
                "tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,"
            ],
            matches
        );

        Ok(())
    }

    #[test]
    fn read_lines() -> io::Result<()> {
        let file = create_file()?;
        let file = File::open(file.path())?;
        let mut count = 0;
        for line in BufReader::new(file).lines() {
            count += 1;
            let line = line?;
            println!("{}: {}", count, line);
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

    /// The writeln! macro is the same as println!, except for two differences:
    /// 1. It takes an extra first argument, a writer.
    /// 2. It returns a Result, so errors must be handled.
    ///
    /// The println! macro doesn't return a Result; it simply panics if the write fails. Since
    /// it writes to the terminal, this is rare.
    #[test]
    fn writers() -> io::Result<()> {
        writeln!(io::stderr(), "hello, world!")?;
        io::stderr().flush()?;

        Ok(())
    }

    /// When a BufWriter is dropped, all remaining buffered data is written to the underlying
    /// writer. However, if an error occurs during this write, the error is ignored. (Since this
    /// happens inside BufWriter’s .drop() method, there is no useful place to report the error.) To
    /// make sure your application notices all output errors, manually .flush() buffered writers
    /// before dropping them.
    #[test]
    fn bufwriter() -> io::Result<()> {
        let file = File::create("/tmp/tmp.txt")?;
        let mut writer = BufWriter::new(file);
        let data = String::from("hello, world");
        writer.write_all(data.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    ///
    #[test]
    fn seeking() -> io::Result<()> {
        let file = create_file()?;
        let mut file = File::open(file.path())?;
        file.seek(SeekFrom::Start(10))?;
        let mut buffer = [0u8; 5];
        file.read_exact(&mut buffer)?;

        assert_eq!(b"m dol", &buffer);
        Ok(())
    }
}
