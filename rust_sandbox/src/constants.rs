/// Rust has two different types of constants which can be declared in any scope including global.
/// Both require explicit type annotation:
///
/// 1.  const: An unchangeable value (the common case).
/// 2.  static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred
///     and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.

static LANGUAGE: &str = "Rust";
// Accessing or modifying a mutable static variable is "unsafe"!
pub static mut COUNT: u32 = 0;
const THRESHOLD: i32 = 10;

pub fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
pub fn is_favorite_language(lang: &str) -> bool {
    lang == LANGUAGE
}

#[cfg(test)]
mod tests {
    use crate::constants::{is_big, is_favorite_language, LANGUAGE};

    #[test]
    fn test_is_big() {
        assert_eq!(true, is_big(11));
        assert_eq!("Rust", LANGUAGE);
    }

    #[test]
    fn test_local_constant() {
        const LOCAL_COUNT: u32 = 99;
        assert_eq!(99, LOCAL_COUNT);
    }

    #[test]
    fn test_local_static() {
        static NAME: &str = "John";
        assert_eq!("John", NAME);
    }

    #[test]
    fn test_is_favorite_language() {
        assert!(is_favorite_language("Rust"));
    }
}
