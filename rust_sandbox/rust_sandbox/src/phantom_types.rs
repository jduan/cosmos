/// A phantom type parameter is one that doesn't show up at runtime, but is checked
/// statically (and only) at compile time.
/// Data types can use extra generic type parameters to act as markers or to perform
/// type checking at compile time. These extra parameters hold no storage values, and
/// have no runtime behavior.
use std::marker::PhantomData;

#[derive(PartialEq, Debug)]
pub struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq, Debug)]
pub struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::phantom_types::{PhantomStruct, PhantomTuple};

    #[test]
    #[allow(unused_variables)]
    fn test_phantom_types() {
        let t1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
        let t2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
        // This line doesn't compile because of type mismatch.
        // assert_eq!(t1, t2);

        let s1: PhantomStruct<&str, f32> = PhantomStruct {
            first: "hello",
            phantom: PhantomData,
        };
        let s2: PhantomStruct<&str, f64> = PhantomStruct {
            first: "world",
            phantom: PhantomData,
        };
        // same here, this line doesn't compile because of type mismatch
        // assert_eq!(s1, s2);
    }
}
