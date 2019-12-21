/// There are 3 functions that can iterate through a collection and they return different views:
/// iter: borrows each element of the collection through each iteration
/// iter_mut: mutably borrows each element of the collection through each iteration
/// into_iter: consumes each element of the collection through each iteration
use std::iter::Iterator;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names {
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
}
