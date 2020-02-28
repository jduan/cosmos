/// When dealing with resources, the default behavior is to transfer them during assignments or function calls. However, sometimes we need to make a copy of the resource as well.
///
/// The Clone trait helps us do exactly this. Most commonly, we can use the .clone() method defined by the Clone trait.
#[derive(Debug, Clone)]
pub struct Pair(Box<i32>, Box<i32>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let pair = Pair(Box::new(1), Box::new(2));
        let moved_pair = pair;

        // Can't access "pair" anymore due to move
        //        println!("Original pair: {:?}", pair);

        let cloned_pair = moved_pair.clone();
        // You can access both moved_pair and cloned_pair!
        assert_eq!(cloned_pair.0, Box::new(1));
        assert_eq!(moved_pair.0, Box::new(1));
    }
}
