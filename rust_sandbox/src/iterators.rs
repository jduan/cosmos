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
///
///
/// Free functions:
/// 1. std::iter::empty()           returns None immediately
/// 2. std::iter::once(5)           produces the given value, and then ends
/// 3. std::iter::repeat("hello")   produces the given value forever
///
///
/// ## Adapter Methods
///
/// Once you have an iterator in hand, the Iterator trait provides a broad selection of
/// "adapter methods", that consume one iterator and build a new one with useful behaviors.
/// such as:
/// * map
/// * filter
/// * flat_map
/// * take
/// * scan
/// * and a lot more ...
///
/// There are 2 important points to notice about iterator adapters.
///
/// 1. First, simply calling an adapter on an iterator doesn't consume any items; it just returns
/// a new iterator, ready to produce its own items by drawing from the first iterator as needed.
/// In other words, iterators are "lazy"! Common ways of consuming iterators are calling "next"
/// or "collect" on the iterator.
///
/// 2. Secondly, iterator adapters are a zero-overhead abstraction. Since map, filter, and
/// their companions are generic, applying them to an iterator specializes their code for the
/// specific iterator type involved. This means that Rust has enough information to inline each
/// iterator’s next method into its consumer, and then translate the entire arrangement into machine
/// code as a unit. So the lines/map/filter chain of iterators we showed before is as efficient as
/// the code you would probably write by hand.
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
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};
    use std::str::FromStr;

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
    /// fold is really general. Many of the other methods for consuming an iterator's values
    /// can be written as uses of fold.
    fn test_fold() {
        let n = 10;
        let sum = (1..n + 1).fold(0, |sum, elem| sum + elem);
        assert_eq!(55, sum);

        let a = [5, 6, 7, 8, 9, 10];
        // use fold to implement max
        assert_eq!(
            10,
            a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m, i))
        );

        let words = [
            "Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs",
        ];
        // The accumulator is moved into and out of the closure, so you can use fold with
        // non-Copy accumlator types, such as String.
        let pangram = words.iter().fold(String::new(), |mut s, &w| {
            s.push_str(w);
            s
        });
        assert_eq!("Pack my box with five dozen liquor jugs", pangram);
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

    #[test]
    fn drain_collection() {
        let mut outer = vec![
            String::from("John"),
            String::from("Dave"),
            String::from("Ava"),
            String::from("Luke"),
            String::from("Tom"),
        ];
        let mut inner = vec![];
        // If you need to drain the whole collection, use the full range ".."
        for mut s in outer.drain(1..3) {
            s.insert(s.len(), '!');
            inner.push(s);
        }
        assert_eq!(vec![String::from("Dave!"), String::from("Ava!"),], inner);
        assert_eq!(
            vec![
                String::from("John"),
                String::from("Luke"),
                String::from("Tom"),
            ],
            outer
        );
    }

    #[test]
    /// Vec<T> and &[T] have various fancy methods to iterate through their elements.
    fn vector_iterators() {
        let names = vec![
            String::from("John"),
            String::from("Dave"),
            String::from("Ava"),
            String::from("Luke"),
            String::from("Tom"),
        ];

        // "windows" produces every contiguous slice of the given length. The windows overlap.
        let windows = names.windows(2);
        for pair in windows {
            assert_eq!(2, pair.len());
            println!("pair: {:?}", pair);
        }

        // "chunks" produces non-overlapping, contiguous slice of the given length.
        // there's also "chunks_mut"
        let chunks = names.chunks(2);
        for chunk in chunks {
            println!("chunk: {:?}", chunk);
        }

        // There's also split_mut and rsplit
        let str = "hello,world,everyone";
        // Patterns can be many things: characters, strings, closures.
        for part in str.split(',') {
            println!("part: {}", part);
        }

        let v = str.split(|c| c == ',').collect::<Vec<&str>>();
        assert_eq!(v, ["hello", "world", "everyone"]);

        let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
        assert_eq!(v, ["abc", "def", "ghi"]);

        // Strings have even more methods: bytes, chars, split_whitespace(), lines, split, matches
        let paragraph = r###"
        line one
        line two
        "###
        .trim();
        let mut lines = paragraph.lines();
        assert_eq!("line one", lines.next().unwrap().trim());
        assert_eq!("line two", lines.next().unwrap().trim());
    }

    #[test]
    fn map_and_filter() {
        let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
        let v: Vec<&str> = text.lines().map(str::trim).collect();
        assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

        // There are 3 iterators at play here: lines, map, and filter
        // The closures taken by map and filter are different:
        // 1. A map iterator passes each item to its closure by value, and in turn, passes along
        // ownership of the closure's result to its consumer
        // 2. A filter iterator passes each item to its closure by shared reference, retaining
        // ownership in case the item is selected to be passed on to its consumer
        let v2: Vec<&str> = text
            .lines()
            .map(str::trim)
            // s is a ref to the vector's element and the vector's elements are &str themselves
            .filter(|s| *s != "iguanas")
            .collect();
        assert_eq!(v2, ["ponies", "giraffes", "squid"]);
    }

    #[test]
    fn filter_map() {
        let text = "1\nfrond .25  289\n3.1415 estuary\n";
        let numbers: Vec<f64> = text
            .split_whitespace()
            // filter_map takes a closure that returns an Option type, dropping all the None values
            .filter_map(|w| f64::from_str(w).ok())
            .collect();

        // Same as above but it's a bit ungainly
        let numbers2: Vec<f64> = text
            .split_whitespace()
            .map(f64::from_str)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
        assert_eq!(vec![1.0, 0.25, 289.0, 3.1415], numbers);
        assert_eq!(vec![1.0, 0.25, 289.0, 3.1415], numbers2);
    }

    #[test]
    fn scan() {
        let squares: Vec<i32> = (0..10)
            // sum is the internal state
            // iteration ends when None is returned from the closure
            .scan(0, |sum, item| {
                *sum += item;
                if *sum > 10 {
                    None
                } else {
                    Some(item * item)
                }
            })
            .collect();

        assert_eq!(vec![0, 1, 4, 9, 16], squares);
    }

    #[test]
    fn take_while() {
        let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
        let headers: Vec<&str> = message
            .lines()
            .take_while(|line| !line.is_empty())
            .collect();
        assert_eq!(
            vec!["To: jimb", "From: superego <editor@oreilly.com>"],
            headers
        );
    }

    #[test]
    fn skip_while() {
        let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
        let body: Vec<&str> = message
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1) // skip the empty line
            .collect();
        assert_eq!(
            vec![
                "Did you get any writing done today?",
                "When will you stop wasting time plotting fractals?"
            ],
            body
        );
    }

    #[test]
    /// You can turn almost any iterator into a peekable iterator by calling the Iterator
    /// trait's peekable method.
    /// Calling peek tries to draw the next item from the underlying iterator, and if there is one,
    /// caches it until the next call to next. All the other Iterator methods on Peekable know about
    /// this cache.
    fn peekable() {
        let mut chars = "47328943789243,4378294732".chars().peekable();
        let mut n: u128 = 0;
        loop {
            match chars.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap() as u128;
                }
                _ => {
                    break;
                }
            }

            chars.next();
        }
        assert_eq!(47328943789243 as u128, n);
    }

    #[test]
    /// Once an Iterator has returned None, the trait doesn’t specify how it ought to behave if you
    /// call its next method again. Most iterators just return None again, but not all.
    ///
    /// The fuse adapter takes any iterator and turns into one that will definitely continue to
    /// return None once it has done so the first time.
    fn fuse_iterator() {
        struct Flaky(bool);

        impl Iterator for Flaky {
            type Item = &'static str;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("totally the last item")
                } else {
                    self.0 = true; // D'oh!
                    None
                }
            }
        }

        let mut flaky = Flaky(true);
        assert_eq!(Some("totally the last item"), flaky.next());
        assert_eq!(None, flaky.next());
        assert_eq!(Some("totally the last item"), flaky.next());

        let mut not_flaky = Flaky(true).fuse();
        assert_eq!(Some("totally the last item"), not_flaky.next());
        assert_eq!(None, not_flaky.next());
        assert_eq!(None, not_flaky.next());
        assert_eq!(None, not_flaky.next());
        assert_eq!(None, not_flaky.next());
    }

    #[test]
    /// trait DoubleEndedIterator: Iterator
    /// The standard library provides double-ended iteration whenever it's practical.
    /// eg: BTreeSet and BTeeMap are double-ended too.
    fn double_ended_iterator() {
        let parts = ["head", "thorax", "abdomen"];
        let mut iter = parts.iter();
        assert_eq!(Some(&"head"), iter.next());
        assert_eq!(Some(&"abdomen"), iter.next_back());
        assert_eq!(Some(&"thorax"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());

        // If an iterator is double-ended, you can reverse it with the "rev" adapter.
        let meals = ["breakfast", "lunch", "dinner"];

        let mut iter = meals.iter().rev();
        assert_eq!(iter.next(), Some(&"dinner"));
        assert_eq!(iter.next(), Some(&"lunch"));
        assert_eq!(iter.next(), Some(&"breakfast"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    /// The inspect() adapter simply applies a closure to a shared reference to each item, and
    /// then passes the item through. The closure can’t affect the items, but it can do things
    /// like print them or make assertions about them.
    fn inspect() {
        let upper_case: String = "hello"
            .chars()
            .inspect(|ch| println!("before: {}", ch))
            // The uppercase equivalent of the lowercase German letter “ß” is “SS”, which is why
            // char::to_uppercase returns an iterator over characters, not a single replacement
            // character.
            .flat_map(char::to_uppercase)
            .inspect(|ch| println!("after: {}", ch))
            .collect();
        assert_eq!("HELLO", upper_case);
    }

    #[test]
    /// You can chain an interator with any iterable that produces the same item type.
    fn chain() {
        let nums: Vec<i32> = (1..=3).chain(vec![4, 5, 6]).collect();
        assert_eq!(vec![1, 2, 3, 4, 5, 6], nums);

        // A chain iterator is reversible if both of its underlying iterators are
        let nums: Vec<i32> = (1..=3).chain(vec![4, 5, 6]).rev().collect();
        assert_eq!(vec![6, 5, 4, 3, 2, 1], nums);
    }

    #[test]
    fn enumerate() {
        let names = ["John", "Jack", "Joe"];
        let pairs: Vec<(usize, &&str)> = names.iter().enumerate().collect();
        assert_eq!(vec![(0, &"John"), (1, &"Jack"), (2, &"Joe")], pairs);
        for (idx, name) in names.iter().enumerate() {
            println!("{}: {}", idx, name);
        }
    }

    #[test]
    /// "zip" is more general than "enumerate"
    fn zip() {
        let names = vec!["John", "Jack", "Joe"];
        for (idx, name) in (0..).zip(names) {
            println!("{}: {}", idx, name);
        }
    }

    #[test]
    /// Throughout this section, we’ve been attaching adapters to iterators. Once you’ve done so,
    /// can you ever take the adapter off again? Usually, no: adapters take ownership of the
    /// underlying iterator, and provide no method to give it back.
    ///
    /// An iterator’s by_ref method borrows a mutable reference to the iterator, so that you can
    /// apply adaptors to the reference. When you’re done consuming items from these adaptors,
    /// you drop them, the borrow ends, and you regain access to your original iterator.
    fn by_ref() {
        let message = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";

        // lines is an iterator
        let mut lines = message.lines();

        let headers: Vec<&str> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        // Now you can still use the "lines" iterator
        let body: Vec<&str> = lines.collect();
        assert_eq!(vec!["To: jimb", "From: id"], headers);
        assert_eq!(vec!["Oooooh, donuts!!"], body);
    }

    #[test]
    /// The cloned adapter takes an iterator that produces references, and returns an iterator
    /// that produces values cloned from those references.
    fn cloned() {
        let names = vec!["John", "Jack", "Joe"];
        let names2: Vec<&str> = names.iter().cloned().collect();
        assert_eq!(names, names2);
    }

    #[test]
    /// The underlying iterator must implement std::clone::Clone, so that cycle can save its
    /// initial state and reuse it each time the cycle starts again.
    fn cycle() {
        let dirs = ["North", "East", "South", "West"];
        let mut spin = dirs.iter().cycle();
        assert_eq!(Some(&"North"), spin.next());
        assert_eq!(Some(&"East"), spin.next());
        assert_eq!(Some(&"South"), spin.next());
        assert_eq!(Some(&"West"), spin.next());
        assert_eq!(Some(&"North"), spin.next());
        assert_eq!(Some(&"East"), spin.next());
        assert_eq!(Some(&"South"), spin.next());
    }

    #[test]
    /// Of course, you can consume an iterator with a for loop, or call next explicitly, but
    /// there are many common tasks that you shouldn’t have to write out again and again.
    fn consume_iterators() {
        // Simple Accumulation: count, sum, product
        // You can extend sum and product to work with other types by implementing the
        // std::iter::Sum and std::iter::Product traits
        let count = (1..100).filter(|n| n % 15 == 0).count();
        assert_eq!(6, count);
        let sum = (1..100).filter(|n| n % 15 == 0).sum();
        assert_eq!(315, sum);

        // max, min, max_by, min_by, max_by_key, min_by_key
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
        let numbers = [1.0, 4.0, 2.0];
        assert_eq!(
            Some(&4.0),
            numbers.iter().max_by(|m, n| m.partial_cmp(n).unwrap())
        );
        let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
        assert_eq!(
            Some(&4.0),
            // We ignore NAN
            numbers.iter().max_by(|m, n| {
                if m.is_nan() {
                    Ordering::Less
                } else if n.is_nan() {
                    Ordering::Greater
                } else {
                    m.partial_cmp(n).unwrap()
                }
            })
        );

        // max_by_key, min_by_key
        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776);
        populations.insert("Fossil", 449);
        populations.insert("Greenhorn", 2);
        populations.insert("Boring", 7_762);
        populations.insert("The Dalles", 15_340);
        assert_eq!(
            Some((&"Portland", &583_776)),
            populations.iter().max_by_key(move |&(_key, value)| value)
        );

        // compare item sequences: eq, lt, le, gt, ge
        let packed = "Helen of Troy";
        let spaced = "Helen   of    Troy";
        let obscure = "Helen of Sandusky"; // nice person, just not famous
        assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
        assert!(spaced < packed);
        assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));

        // any, all
        assert!("Iterator".chars().any(char::is_uppercase));
        assert!(!"Iterator".chars().all(char::is_uppercase));

        // position, rposition
        let text = "Xerxes";
        assert_eq!(Some(1), text.chars().position(|ch| ch == 'e'));
        assert_eq!(None, text.chars().position(|ch| ch == 'z'));
        // rposition requires a reversible iterator. It also requires an
        // std::iter::ExactSizeIterator
        let bytes = b"Xerxes";
        assert_eq!(Some(4), bytes.iter().rposition(|&ch| ch == b'e'));
        assert_eq!(None, bytes.iter().rposition(|&ch| ch == b'z'));

        // nth
        // Note that all preceding elements, as well as the returned element, will be
        // consumed from the iterator. That means that the preceding elements will be
        // discarded.
        let mut squares = (0..10).map(|i| i * i);
        assert_eq!(Some(16), squares.nth(4));
        assert_eq!(Some(25), squares.nth(0));
        assert_eq!(None, squares.nth(6));

        // last
        let squares = (0..10).map(|i| i * i);
        assert_eq!(Some(81), squares.last());

        // find
        assert_eq!(
            None,
            populations.iter().find(|&(_name, &pop)| pop > 1_000_000)
        );
        assert_eq!(
            Some((&"Portland", &583_776)),
            populations.iter().find(|&(_name, &pop)| pop > 500_000)
        );
    }

    #[test]
    /// "collect" isn't specific to vectors; it can build any kind of collection from Rust's
    /// standard library, as long as the iterator produces a suitable item type.
    /// The target type needs to implement the std::iter::FromIterator.
    fn build_collections() {
        let names = vec!["John", "Jack", "Joe"];

        let set: HashSet<&&str> = names.iter().collect();
        assert!(set.contains(&"John"));
        let bset: BTreeSet<&&str> = names.iter().collect();
        assert!(bset.contains(&"John"));
        let list: LinkedList<&&str> = names.iter().collect();
        assert!(list.contains(&&"John"));
        let map: HashMap<&&str, usize> = names.iter().zip(0..).collect();
        assert!(map.contains_key(&&"John"));
    }

    #[test]
    /// If a type implements the std::iter::Extend trait, then its extend method adds an iterable’s
    /// items to the collection. All of the standard collections implement this trait, so does
    /// String. Arrays and slices, which have a fixed length, do not.
    fn the_extend_trait() {
        let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
        v.extend(&[31, 57, 99, 163]);
        assert_eq!(vec![1, 2, 4, 8, 16, 31, 57, 99, 163], v);
    }

    #[test]
    fn partition() {
        let things = [
            "doorknob",
            "mushroom",
            "mushroom",
            "noodle",
            "giraffe",
            "grapefruit",
        ];
        let (short, long): (Vec<&str>, Vec<&str>) = things.iter().partition(|name| name.len() < 8);
        assert_eq!(vec!["noodle", "giraffe"], short);
        assert_eq!(vec!["doorknob", "mushroom", "mushroom", "grapefruit"], long);

        // Like collection, partition can make any sort of collections you like, but both
        // must be of the same type!
        let (short2, long2): (HashSet<&str>, HashSet<&str>) =
            things.iter().partition(|name| name.len() < 8);
        assert_eq!(2, short2.len());
        // "mushroom" is only counted once because of set
        assert_eq!(3, long2.len());
    }

    #[test]
    fn flat_map() {
        let mut major_cities = HashMap::new();
        major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
        major_cities.insert("The United States", vec!["Portland", "Nashville"]);
        major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
        major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
        major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

        let countries = ["Japan", "Brazil", "Kenya"];

        let cities: Vec<&&str> = countries
            .iter()
            .flat_map(|country| major_cities.get(country).unwrap())
            .collect();

        assert_eq!(
            vec![
                &"Tokyo",
                &"Kyoto",
                &"São Paulo",
                &"Brasília",
                &"Nairobi",
                &"Mombasa"
            ],
            cities
        );
    }
}
