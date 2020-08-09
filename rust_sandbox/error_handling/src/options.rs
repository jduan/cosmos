/// Combinators make using types like Option ergonomic because they reduce explicit case analysis.
/// They are also composable because they permit the caller to handle the possibility of absence
/// in their own way. Methods like unwrap remove choices because they will panic if Option<T> is
/// None.

#[allow(unused)]
pub fn find_needle(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

#[allow(unused)]
pub fn get_file_extension(filename: &str) -> Option<&str> {
    match find_needle(filename, '.') {
        None => None,
        Some(i) => Some(&filename[i + 1..]),
    }
}

#[allow(unused)]
// Use the "map" combinator
pub fn get_file_extension2(filename: &str) -> Option<&str> {
    find_needle(filename, '.').map(|i| &filename[i + 1..])
}

#[allow(unused)]
// Use the "map_or" combinator
pub fn get_file_extension3(filename: &str) -> &str {
    find_needle(filename, '.').map_or("rs", |i| &filename[i + 1..])
}

#[allow(unused)]
// Given a filepath, such as "/usr/local/bin/hello.rs", return the extension.
// Use the "and_then" combinator
pub fn get_filepath_extension(filepath: &str) -> Option<&str> {
    let parts = filepath.split('/');

    // You can't use "map" here because you would return an Option of Option.
    parts.last().and_then(|name| get_file_extension(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_needle() {
        assert_eq!(Some(0), find_needle("hello", 'h'));
        assert_eq!(None, find_needle("hello", 'H'));

        // Use the method in user code
        let filename = "foobar.rs";
        match find_needle(filename, '.') {
            None => println!("No file extension found."),
            Some(i) => println!("File extension: {}", &filename[i + 1..]),
        }
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(None, get_file_extension("foobar"));
        assert_eq!(Some("rs"), get_file_extension("foobar.rs"));
    }

    #[test]
    fn test_get_file_extension2() {
        assert_eq!(None, get_file_extension2("foobar"));
        assert_eq!(Some("rs"), get_file_extension2("foobar.rs"));
    }

    #[test]
    fn test_get_file_extension3() {
        assert_eq!("rs", get_file_extension3("foobar"));
        assert_eq!("py", get_file_extension3("foobar.py"));
    }

    #[test]
    fn test_get_filepath_extension() {
        assert_eq!(None, get_filepath_extension("/usr/local/bin"));
        assert_eq!(
            Some("rs"),
            get_filepath_extension("/usr/local/bin/hello.rs")
        );
    }
}
