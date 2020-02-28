pub struct Container(i32, i32);

pub trait Contains<A, B> {
    fn contains(&self, _: A, _: B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, a: i32, b: i32) -> bool {
        a == self.0 && b == self.1
    }

    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

pub fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

/// The use of "Associated types" improves the overall readability of code by moving
/// inner types locally into a trait as output types.

pub struct Container2(i32, i32);

pub trait Contains2 {
    // Define generic types here which methods will be able to utilize.
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains2 for Container2 {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    fn contains(&self, a: &i32, b: &i32) -> bool {
        self.0 == *a && self.1 == *b
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

pub fn difference2<C: Contains2>(c: &C) -> i32 {
    c.last() - c.first()
}

#[cfg(test)]
mod tests {
    use crate::associate_types::{difference, difference2, Container, Container2};

    #[test]
    fn test_difference() {
        let container = Container(1, 10);
        assert_eq!(9, difference(&container));
    }
    #[test]

    fn test_difference2() {
        let container2 = Container2(1, 10);
        assert_eq!(9, difference2(&container2));
    }
}
