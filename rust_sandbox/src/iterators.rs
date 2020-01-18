/// ```
/// pub trait Iterator {
///     type Item;
///     fn next(&mut self) -> Option<Self::Item>;
///     ... and many other functions
/// }
/// ```
///
/// If there's a natural way to iterate over some type, it can implement `std::iter::IntoIterator`, whose `into_iter` method takes a value and returns an iterator over it.
///
/// ```
/// pub trait IntoIterator where Self::IntoIter::Item == Self::Item {
///     type Item;
///     type IntoIter: Iterator;
///     fn into_iter(self) -> Self::IntoIter;
/// }
/// ```
///
/// There are 3 functions that can iterate through a collection and they return different views:
/// 1. iter: borrows each element of the collection through each iteration
/// 2. iter_mut: mutably borrows each element of the collection through each iteration
/// 3. into_iter: consumes each element of the collection through each iteration
///
/// Note that slices like &[T] and &str have "iter" and "iter_mut" methods too.
///
/// Most collections actually provide several implementations of IntoIterator:
/// 1. Given a shared ref to the collection, "into_iter" returns an interator that produces
/// shared refs to its items. For example: (&favoriates).into_iter()
/// 2. Given a mutable reference to the collection, into_iter returns an iterator
/// that produces mutable references to the items. For example: (&mut favorites).into_iter()
/// 3. When passed the collection by value, into_iter returns an iterator that takes
/// ownership of the collection and returns items by value; the items’ ownership moves
/// from the collection to the consumer, and the original collection is consumed in
/// the process. For example: favorites.into_iter()
///
/// These three implementations are what create the following idioms for iterating over
/// a collection in for loops:
///
///     for element in &collection { ... }
///     for element in &mut collection { ... }
///     for element in collection { ... }
///
/// Not every type provides all three implementations. For example, HashSet, BTreeSet
/// and BinaryHeap don’t implement IntoIterator on mutable references, since modifying
/// their elements would probably violate the type’s invariants.
///
/// Slices implement two of the three IntoIterator variants; since they don’t own their
/// elements, there is no “by value” case.
///
/// You may have noticed that the first two IntoIterator variants are equivalent to
/// calling iter() or iter_mut(). Why does Rust provide both?
///
/// IntoIterator is what makes for loops work, so that’s obviously necessary. But when
/// you’re not using a for loop, favorites.iter() is clearer than (&favorites).into_iter().
/// So iter and iter_mut are still valuable for their ergonomics.
///
/// One important thing: iter() and iter_mut() aren't methods of traits. Most iterable
/// types just happen to have methods by those names!
use std::iter::Iterator;

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_range() {
        let mut sum = 0;
        let n = 10;
        // Although a "for" loop always calls "into_iter" on its operand, you can also pass
        // iterators to "for" loops directly; this occurs when you loop over a Range.
        // All iterators automatically implement IntoIterator, with an "into_iter" method
        // that simply returns the iterator itself.
        for i in 1..n + 1 {
            sum += i;
        }

        assert_eq!(55, sum);
    }

    #[test]
    fn iterator_vs_iterable() {
        // Range implements Iterator so we can call Iterator methods on it directly.
        let mut r = 1..10;
        assert_eq!(1, r.next().unwrap());

        // Vector doesn't implement Iterator but it implements IntoIterator so you need to
        // call "into_iter" first before you can call other Iterator methods.
        let names = vec!["john", "dave"];
        let mut iter = names.into_iter();
        assert_eq!("john", iter.next().unwrap());
    }

    #[test]
    fn test_fold() {
        let n = 10;
        let sum = (1..n + 1).fold(0, |sum, elem| sum + elem);
        assert_eq!(55, sum);
    }

    #[test]
    fn test_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in &names {
            println!("name is {}", name);
        }

        // Under the hood, every "for" loop is just shorthand for calls to IntoIterator
        // and Iterator methods.

        let mut iter = (&names).into_iter();
        while let Some(name) = iter.next() {
            println!("name is {}", name);
        }
    }

    #[test]
    fn test_into_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.into_iter() {
            println!("name is {}", name);
        }
        // The line below won't compile.
        // println!("names are {:?}", names);
    }

    #[test]
    fn test_iter_mut() {
        let mut scores = vec![1, 2, 3];
        for score in scores.iter_mut() {
            *score += 10;
        }
        assert_eq!(11, scores[0]);
        assert_eq!(12, scores[1]);
        assert_eq!(13, scores[2]);
    }

    #[test]
    fn iterate_vector() {
        let v1 = vec![1, 2, 3];

        // iterators are lazy. No iteration happens at this time yet.
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[test]
    fn iterate_manually() {
        let v1 = vec![1, 2, 3];

        // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes
        // internal state that the iterator uses to keep track of where it is in the sequence. In other
        // words, this code consumes, or uses up, the iterator. Each call to next eats up an item from
        // the iterator.
        // We didn’t need to make v1_iter mutable when we used a for loop because the
        // loop took ownership of v1_iter and made it mutable behind the scenes.
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        // Note that you can't use the v1_iter anymore because sum takes ownership of it.
        // However, you can still access the original vector.
        assert_eq!(1, v1[0]);
    }

    #[test]
    fn iterator_adaptors() {
        let v1 = vec![1, 2, 3];
        // Without the call of collect(), the new iterator won't be consumed.
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_and_closure() {
        let v1 = vec![1, 2, 3, 4, 5];
        let target = 3;
        // into_iter() creates an iterator that takes owernship of v1 and returns owned values.
        let v2: Vec<_> = v1.into_iter().filter(|n| n >= &target).collect();
        assert_eq!(v2, vec![3, 4, 5]);
    }

    #[test]
    fn implement_iterator() {
        let counter = Counter::new();
        // let nums: Vec<_> = counter.into_iter().collect();
        for num in counter.into_iter() {
            println!("next count: {}", num);
        }

        // You can use other methods that are provided by the Iterator interface by default.
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("The sum is {}", sum);
        assert_eq!(18, sum);
    }

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(Some(1), fib.next());
        assert_eq!(Some(1), fib.next());
        assert_eq!(Some(2), fib.next());
        assert_eq!(Some(3), fib.next());
        assert_eq!(Some(5), fib.next());
    }

    #[test]
    fn iterate_arrays() {
        let array = [1u32, 3, 3, 7];
        // The `iter` method produces an `Iterator` over an array/slice.
        for n in array.iter() {
            println!("next element of array is {}", n);
        }
    }

    #[test]
    fn iterate_hashmap() {
        let mut people = HashMap::new();
        people.insert("John", 30);
        people.insert("Dave", 50);

        for (key, value) in &mut people {
            // Refs to keys are immutable
            println!("name: {}", key);
            // Refs to values are mutable
            *value += 1;
        }

        assert_eq!(&31, people.get("John").unwrap());
        assert_eq!(&51, people.get("Dave").unwrap());

        let mut colors = HashSet::new();
        colors.insert("red");
        colors.insert("green");
        // This code doesn't compile.
        //        for color in &mut colors {
        //            println!("color is {}", color);
        //        }
    }
}
