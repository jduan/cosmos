#[cfg(test)]
mod tests {
    #[test]
    fn test_literals() {
        // Suffixed literals, their types are known at initialization
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        // Unsuffixed literals, their types depend on how they are used
        let i = 1; // defaults to i32
        let f = 1.0; // defaults to f64

        assert_eq!(1, std::mem::size_of_val(&x));
        assert_eq!(4, std::mem::size_of_val(&y));
        assert_eq!(4, std::mem::size_of_val(&z));
        assert_eq!(4, std::mem::size_of_val(&i));
        assert_eq!(8, std::mem::size_of_val(&f));
    }
}
