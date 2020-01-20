/// There are a few systematic differences between Rust's collections and those in other langs.
///
/// 1. Moves and borrowing are everywhere. Rust uses moves to avoid deep-copying values. That's
/// why the method "Vec<T>::push(item)" takes its argument by values, not by reference.
/// 2. Rust doesn't have invalidation errors -- the kind of dangling-pointer bug where a
/// collection is resized, or otherwise changed, while the program is holding a pointer to
/// data inside it. Rust's borrow checker rules them out at compile time.
/// 3. Rust doesn't have null, so we'll see Options in places where other langs would use null.
///
/// Standard collections (all of them are generic types)
/// * Vec<T>
/// * VecDeque<T>           double-ended queue
/// * LinkedList<T>
/// * BinaryHeap<T>         max heap
/// * HashMap<K, V>
/// * BTreeMap<K, V>
/// * HashSet<T>
/// * BTreeSet<T>
///
/// Out of those: Vec, HashMap, and HashSet are the most useful. The rest have niche uses.
///
/// All collections implement std::iter::FromIterator, so you can create them from an iterator
/// using the "collect()" method.
#[cfg(test)]
mod tests {
    use std::collections::{BinaryHeap, LinkedList, VecDeque};

    #[test]
    /// A vector has 3 fields:
    /// * the length,
    /// * the capacity,
    /// * a pointer to a heap allocation where the elements are stored
    ///
    /// Arrays, slices, and vectors share a lot of the same methods.
    /// However, arrays and slices are fix-sized so they don't grow or shrink!
    fn vec() {
        // accessing elements
        let nums = vec![0, 1, 2, 3, 4];
        let lines = vec![
            String::from("line one"),
            String::from("line two"),
            String::from("line three"),
            String::from("line four"),
        ];
        assert_eq!(4, nums[4]);
        let line2 = lines[1].clone(); // requires clone
        let line2_ref = &lines[1];
        assert_eq!(String::from("line two"), line2);
        assert_eq!("line two", line2_ref);
        let my_ref = &nums[1..3]; // get a ref to a slice
        assert_eq!(&[1, 2], my_ref);
        let my_copy = lines[1..3].to_vec(); // get a copy of a slice, requires Clone
        assert_eq!(
            vec![String::from("line two"), String::from("line three")],
            my_copy
        );

        let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let empty_vec: Vec<i32> = vec![];

        // first
        assert_eq!(Some(&0), nums.first());
        assert_eq!(None, empty_vec.first());

        // last
        assert_eq!(Some(&7), nums.last());
        assert_eq!(None, empty_vec.last());

        // get(index)
        assert_eq!(Some(&3), nums.get(3));
        let n = nums[3];
        assert_eq!(3, n);
        // Panic if an index is out of bounds!
        // let n = nums[30];
        assert_eq!(None, nums.get(30));

        // first_mut, last_mut, get_mut
        let last = nums.last_mut().unwrap();
        *last *= 10;
        assert_eq!(Some(&70), nums.last());

        // to_vec: makes copies
        let v = [1, 2, 3, 4, 5];
        assert_eq!(vec![1, 2, 3, 4, 5], v.to_vec());
        assert_eq!(vec![1, 2, 3], v[0..3].to_vec());

        // len, is_empty

        // You can explicitly manage capacity of a vector if you want to:
        // Vec::with_capacity(n)
        // vec.capacity() returns the vec's capacity
        // vec.reserve(n) makes sure the vector has at least enough spare capacity
        //      for "n more elements"
        // vec.reserve_exact(n)
        // vec.shrink_to_fix() tries to free up the extra memory if vec.capacity()
        //      is greater than vec.len().

        // add or remove elemens: push, pop, insert(index, value), remove(index)

        // add or remove many values at once:
        // extend(iterable),
        // split_off(index),
        // append(vec2),
        // drain(range)

        // oddball methods for selectively removing some elements:
        // retain(test): remove all elements that don't pass the given test
        // dedup: drops repeated, adjacent elements
        // dedup_by(same): takes a closure
        // dedup_by_key(key)

        // concat
        assert_eq!(vec![1, 2, 3, 4, 5, 6], [[1, 2], [3, 4], [5, 6]].concat());

        // join
        let names = ["Beijing", "Shanghai", "Tianjin"];
        assert_eq!("Beijing Shanghai Tianjin", names.join(" "));

        // slices
        let mut v = vec![0, 1, 2, 3];
        assert_eq!(&1, &v[1]);
        assert_eq!(&[0, 1], &v[..2]);
        assert_eq!(&[2, 3], &v[2..]);

        // you can only get one mutable ref at a time!
        let a = &mut v[0];
        // this line won't compile
        // let b = &mut v[1];
        *a += 1;
        assert_eq!(1, v[0]);

        // Rust has several methods that can borrow mut references to two or more parts of an
        // array, slice, or vector at once. Unlike the code above, these methods are safe,
        // because by design, they split the data into nonoverlapping regions.
        // There are other methods:
        // split_first, split_first_mut, split_last, split_last_mut
        // splitn(n, is_sep), splitn_mut(n, is_sep), rsplitn(n, is_sep), rsplitn_mut(n, is_sep)
        // chunks(n), chunks_mut(n), windows(n)
        let (left, right) = v.split_at_mut(2);
        assert_eq!(&[1, 1], left);
        right[1] += 1;
        assert_eq!(&[2, 4], right);
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        for parts in nums.split_mut(|&num| num % 3 == 0) {
            if !parts.is_empty() {
                parts[0] *= 10;
            }
        }
        assert_eq!(vec![10, 2, 3, 40, 5, 6], nums);

        // swapping:
        // swap(i, j)
        // swap_remove(i)   the last element is moved to the removed spot

        // sorting:
        // sort()
        // sort_by(cmp)
        // sort_by_key(key)
        // reverse()    in place

        // once a slice is sorted, it can be efficiently searched:
        // binary_search(&value)
        // binary_search_by(&value, cmp)
        // binary_search_by_key(&value, key)

        // compare slices:
        // If a type T supports == and != (PartialEq), then [T; N], &[T], and Vec<T>
        //  support them too
        // If a type T supports <, <=, > and >= (PartialOrd), then [T; N], &[T], and Vec<T>
        //  support them too
        // slice.starts_with(other_slice)
        // slice.ends_with(other_slice)

        // random elements (you need the "rand" crate)
        // rng.choose(slice)
        // rng.shuffle(slice)
    }

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn rust_rules_out_invalidation_errors() {
        let mut v = vec![1, 3, 5, 7, 9];
        for (idx, &num) in v.iter().enumerate() {
            if num > 4 {
                // cannot borrow `v` as mutable because it is also borrowed as immutable
                // v.remove(idx);
            }
        }

        // You can do this instead
        v.retain(|&val| val <= 4);

        assert_eq!(vec![1, 3], v);
    }

    /// Rust’s std::collections::VecDeque<T> is a deque (pronounced “deck”), a double-ended
    /// queue. It supports efficient add and remove operations at both the front and the back.
    /// VecDeque is implemented using a ring buffer.
    ///
    /// push_front(value)
    /// push_back(value)
    /// pop_front()
    /// pop_back()
    /// front(), front_mut()
    /// back(), back_mut()
    ///
    /// iterators: iter(), iter_mut(), into_iter()
    ///
    /// Because deques don't store their elements contiguously in memory, they don't inherit
    /// all the methods of slices. One way to perform vector and slice operations on deques
    /// is to convert it to a Vec, do the operation, and convert it back:
    ///
    /// Vec<T> implements From<VecDeque<T>>, so Vec::from(deque) turns a deque into a vector
    /// VecDeque<T> implements From<Vec<T>>, so VecDeque::from(vec) turns a vector into a deque
    #[test]
    fn vec_deque() {
        let mut scores = VecDeque::from(vec![1, 2, 3, 4, 5]);
        scores.push_front(0);
        scores.push_back(6);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], Vec::from(scores));
    }

    /// std::collections::LinkedList<T> is a doubly linked list for Rust. It supports a subset
    /// of VecDeque's methods. Methods that access elements by index, though, are generally
    /// omitted, since it’s inherently inefficient to access linked list elements by index.
    ///
    /// For now, the main advantage of LinkedList over VecDeque is that combining two lists is
    /// very fast. list.append(&mut list2), the method that moves all elements from one list to
    /// another, only involves changing a few pointers, which can be done in constant time.
    #[test]
    fn linked_list() {
        let mut lst1 = LinkedList::new();
        lst1.push_back(1);
        lst1.push_back(2);
        lst1.push_back(3);
        let mut lst2 = LinkedList::new();
        lst2.push_back(10);
        lst2.push_back(20);
        lst2.push_back(30);

        // This reuses all the nodes from `lst2` and moves them into `lst1`. After
        // this operation, `lst2` becomes empty.
        lst1.append(&mut lst2);
        assert_eq!(
            vec![1, 2, 3, 10, 20, 30],
            lst1.into_iter().collect::<Vec<i32>>()
        );
        assert!(lst2.is_empty());
    }

    /// A BinaryHeap is a collection whose elements are kept loosely organized so that the
    /// greatest value always bubbles up to the front of the queue.  It can hold any type of
    /// value that implements the Ord trait.
    ///
    /// The most important methods are:
    /// push(value)
    /// pop() returns an Option<T>
    /// peek() returns an Option<&T>
    ///
    /// A good use of BinaryHeap is work queues. You can define a task struct that implements
    /// Ord on the basis of priority, so that higher-priority tasks are Greater than
    /// lower-priority tasks. Then, create a BinaryHeap to hold all pending tasks. Its .pop()
    /// method will always return the most important item, the task your program should work
    /// on next.
    #[test]
    fn binary_heap() {
        let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
        assert_eq!(Some(&9), heap.peek());
        assert_eq!(Some(9), heap.pop());
        assert_eq!(Some(8), heap.pop());
        assert_eq!(Some(6), heap.pop());

        // BinaryHeap is iterable but its iterator produces elements in an arbitrary order.
        // There's also into_iter_sorted() but it's unstable.
        println!("heap: {:?}", heap.iter().collect::<Vec<&i32>>());

        // To consume values from a BinaryHeap in order of priority, use a while loop:
        let mut tasks = BinaryHeap::from(vec![1, 2, 3, 4, 5]);
        while let Some(task) = tasks.pop() {
            println!("task: {}", task);
        }
    }
}
