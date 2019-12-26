use regex::Regex;

/// Prefer to pass &str than String to functions.
/// Prefer to return &str than String from functions.

#[allow(dead_code)]
fn print_me(msg: &str) {
    println!("msg is {}", msg);
}

/// When should you use 'str instead of String in structs?
/// 1. We should use a reference if our struct doesn't own the variable. We are only borrowing it
/// for a while.
/// 2. Is the String large? If it is large, then passing it by reference will save unnecessary
/// memory copy and will be a lot faster.
pub struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    pub fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

/// Sometimes a struct is really the owner of a String.
pub struct Person2 {
    pub name: String,
}

/// Use the "Into" trait to make Person2::new take anything that can be turned into a String.
/// This works for both String and &str.
impl Person2 {
    pub fn new<S>(name: S) -> Person2
    where
        S: Into<String>,
    {
        Person2 { name: name.into() }
    }
}

pub fn run() {
    create_strings();
    update_strings();
    iterate_string();
}

fn create_strings() {
    // this creates an empty string
    let mut s = String::new();
    println!("string is: {}", s);

    let data = "initial contents";
    // to_string is a method available on any type that implements the Display trait
    let s: String = data.to_string();
    println!("string is: {}", s);

    // same as above
    let s = String::from("initial contents");

    // strings are UTF-8 encoded
    let hello = String::from("你好");
    println!("string is: {}", hello);
}

fn update_strings() {
    let mut s = String::from("foo");

    // grow a string
    let s2 = "bar";
    s.push_str(s2);
    println!("string is: {}", s);
    // push_str takes a string slice because we don't necessarily want to take
    // ownership of the parameter. Otherwise, the line below won't compile.
    println!("s2 is: {}", s2);

    // append a single char
    s.push('!');
    println!("string is: {}", s);

    // combine two strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // note that s1 has been "moved" here and can no longer be used
    // The + operator uses the "add" method, whose signature looks like:
    // fn add(self, s: &str) -> String
    // This method takes ownership of s1, appends a copy of the contents of s2,
    // and then returns ownership of the result.
    let s3 = s1 + &s2;

    // this looks ugly:
    // let s = s1 + "-" + &s2 + "-" + &s3;

    // use the "format!" macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("string is: {}", s);

    // format! doesn't take ownership of s1
    println!("s1 is {}", s1);
}

// You can't index into Strings!
// A String is a wrapper over a Vec<u8>. Strings are encoded in UTF-8 format.
// Indexing into a UTF-8 encoded string doesn't really make sense because
// the "byte" for a given index may not make sense at all.
fn access_string() {
    let s = String::from("hello");
    // the line below won't compile!
    // let h = s[0];
}

fn iterate_string() {
    // If you need to perform operations on individual Unicode scalar values,
    // the best way to do so is to use the chars method.
    let s = String::from("你好");
    for c in s.chars() {
        println!("char is {}", c);
    }

    // The bytes method returns each raw byte, which might be appropriate for
    // your domain.
    for b in s.bytes() {
        println!("byte is {}", b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person() {
        let person = Person {
            name: "Donald Duck",
        };
        person.greet();
    }

    #[test]
    fn test_person2() {
        let person2 = Person2::new("Herman");
        assert_eq!("Herman", &person2.name);

        // this works too thanks to the Into trait!
        let person2 = Person2::new(String::from("Herman"));
        assert_eq!("Herman", &person2.name);
    }

    #[test]
    fn string_literals() {
        let speech = "\"Outch!\" said the well.\n";

        println!(
            "A string literal may
            span multiple lines."
        );

        // If one line of a string ends with a backslash, then the newline and the leading
        // whitespace on the line are dropped. So this prints a single line.
        println!(
            "It was a bright, cold day in April, and \
             there were four of us \
             more or less."
        );

        // Raw strings. Everything inside a raw string are included verbatim. No escape sequences
        // are recognized.
        let default_win_install_path = r"C:\Program Files\Gorillas";
        let pattern = Regex::new(r"\d+(\.\d+)*");
    }

    #[test]
    fn byte_strings() {
        // A byte string is a slice of u8 values, ie bytes, rather than UTF8 characters.
        let method: &[u8; 3] = b"GET";
        assert_eq!(method, &[b'G', b'E', b'T']);
    }

    #[test]
    fn ways_to_create_strings() {
        // .to_string() converts a &str to a String. This copies the string.
        let error_message = "too many pets".to_string();

        // format! works just like println! except it doesn't add a newline at the end
        let date = format!("{}-{}-{}", 2019, 11, 11);
        assert_eq!(date, "2019-11-11");

        // arrays, slices, and vectors of strings have two methods (concat and join) that form a
        // new String from many strings.
        let bits = vec!["veni", "vidi", "vici"];
        assert_eq!(bits.concat(), "venividivici");
        assert_eq!(bits.join(", "), "veni, vidi, vici");
    }

    #[test]
    fn test_string_literals() {
        // This is a raw string literal.
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // If you need quotes in a raw string, add a pair of #s
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // If you need "# in your string, just use more #s in the delimiter.
        // There is no limit for the number of #s you can use.
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        print!("{}", longer_delimiter);
    }

    #[test]
    fn test_byte_strings() {
        let bs: &[u8; 21] = b"this is a byte string";
        // Byte strings don't implement the Display trait.
        println!("A byte string: {:?}", bs);

        // Byte strings can have byte escapes...
        let escaped = b"\x52\x75\x73\x74 as bytes";
        println!("Some escaped bytes: {:?}", escaped);

        // Raw byte strings work just like raw strings
        let raw_bytestring = br"\u{211D} is not escaped here";
        println!("Raw byte string: {:?}", raw_bytestring);

        // Converting a byte string to &str can fail
        if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
            println!("And the same as text: {}", my_str);
        } else {
            eprint!("Failed to convert byte string to &str");
        }

        // Byte strings don't have to be UTF-8
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";
        println!("shift_jis: {:?}", shift_jis);
        match std::str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conversion successful: '{}'", my_str),
            Err(e) => println!("Conversion failed: {:?}", e),
        };
    }
}
