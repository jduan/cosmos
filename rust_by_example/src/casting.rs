/// Rust provides no implicit type conversion (coercion) between primitive types. But, explicit
/// type conversion (casting) can be performed using the "as" keyword.
///
/// Rust detects overflowing literals at compile time. This disables that.
/// However, rules for converting between integral types follow C conventions generally, except
/// in cases where C has undefined behavior. The behavior of all casts between integral types is
/// well defined in Rust.
#[allow(overflowing_literals)]
#[cfg(test)]
mod tests {
    #[test]
    fn casting() {
        let decimal = 65.4321_f32;

        // convert a f32 to u8
        let integer = decimal as u8;
        assert_eq!(65, integer);

        // convert a f32 to a char
        let char = integer as char;
        assert_eq!('A', char);

        // when casting any value to an unsigned type, T,
        // std::T::MAX + 1 is added or subtracted until the value
        // fits into the new type
        assert_eq!(232, 1000 as u8);
    }
}
