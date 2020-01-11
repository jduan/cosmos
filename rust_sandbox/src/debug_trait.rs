use std::fmt;
use std::fmt::{Display, Error, Formatter};

/// All types can derive the "std::fmt::Debug" implementation. This is not true for
/// "std::fmt::Display", which must be manually implemented.
///
/// All std library types automatically are printable with {:?} too.
///
/// There are other traits in "std::fmt", such fmt::Binary which is used for {:b}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

/// fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output
/// appearance. This is done by manually implementing fmt::Display.
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person(name: {}, age: {})", self.name, self.age)
    }
}

pub struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[")?;
        let vec = &self.0;
        for (idx, num) in vec.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", num)?;
        }
        write!(f, "]")
    }
}

pub struct City {
    name: &'static str,
    lat: f32, // latitude
    lon: f32, // longitude
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 53.348°N 6.260°W
        let lat_symbol = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_symbol = if self.lon >= 0.0 { 'E' } else { 'W' };
        let degree_symbol = "\u{00B0}";
        write!(
            f,
            "{}: {:.3}{}{} {:.3}{}{}",
            self.name,
            self.lat.abs(),
            degree_symbol,
            lat_symbol,
            self.lon.abs(),
            degree_symbol,
            lon_symbol
        )
    }
}

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Color {{ red: {}, green: {}, blue: {} }}",
            self.red, self.green, self.blue
        )
    }
}

impl Color {
    pub fn display(&self) -> String {
        // Return something like "RGB (128, 255, 90) 0x80FF5A"
        format!(
            "RGB ({}, {}, {}) {}",
            self.red,
            self.green,
            self.blue,
            self.hex_fomrat()
        )
    }

    /// Return something like "0x80FF5A"
    fn hex_fomrat(&self) -> String {
        format!("0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use crate::debug_trait::{City, Color, List, Person};

    #[test]
    fn test_debug() {
        let person = Person {
            name: "Jingjing Duan".to_string(),
            age: 99,
        };
        println!("{:#?}", person); // pretty print
        let debug_str = format!("{:?}", person); // debug print
        assert_eq!(
            debug_str,
            String::from("Person { name: \"Jingjing Duan\", age: 99 }")
        );
    }

    #[test]
    fn test_display() {
        let person = Person {
            name: "Jingjing Duan".to_string(),
            age: 99,
        };
        let display_str = format!("{}", person); // debug print
        assert_eq!(
            display_str,
            String::from("Person(name: Jingjing Duan, age: 99)")
        );
    }

    #[test]
    fn test_display_for_vec() {
        let nums = List(vec![1, 2, 3]);

        assert_eq!("List[1, 2, 3]", format!("{}", nums));
    }

    #[test]
    fn test_city_display() {
        let dublin = City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        };
        assert_eq!("Dublin: 53.348°N 6.260°W", format!("{}", dublin));
        let oslo = City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        };
        assert_eq!("Oslo: 59.950°N 10.750°E", format!("{}", oslo));
        let vancouver = City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        };
        assert_eq!("Vancouver: 49.250°N 123.100°W", format!("{}", vancouver));
    }

    #[test]
    fn test_color_display() {
        let c1 = Color {
            red: 128,
            green: 255,
            blue: 90,
        };
        assert_eq!(
            "Color { red: 128, green: 255, blue: 90 }",
            format!("{}", c1)
        );
        assert_eq!("RGB (128, 255, 90) 0x80FF5A", c1.display());
        let c2 = Color {
            red: 0,
            green: 3,
            blue: 254,
        };
        assert_eq!("Color { red: 0, green: 3, blue: 254 }", format!("{}", c2));
        assert_eq!("RGB (0, 3, 254) 0x0003FE", c2.display());
        let c3 = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        assert_eq!("Color { red: 0, green: 0, blue: 0 }", format!("{}", c3));
        assert_eq!("RGB (0, 0, 0) 0x000000", c3.display());
    }
}
