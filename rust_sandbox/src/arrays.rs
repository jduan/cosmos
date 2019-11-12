#[cfg(test)]
mod tests {
    #[test]
    fn test_arrays() {
        let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

        assert_eq!(lazy_caterer[3], 7);
        assert_eq!(taxonomy.len(), 3);
    }

    #[test]
    fn test_long_arrays() {
        // an array of 10,000 bool elements, all set to true
        // Rust has no notion for an uninitialized array. In general, Rust ensure
        // that code can never access any sort of uninitialized value!
        let mut sieve = [true; 10_000];

        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10_000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        assert!(sieve[211]);
        assert!(!sieve[9876]);
    }

    #[test]
    fn arrays_are_fix_sized() {
        let n = 10;
        // This line doesn't compile because the "size" of the array has to be a constant!
        // let nums = [1; n];
        let nums = [1; 10];
        assert_eq!(nums.len(), 10);
    }
}
