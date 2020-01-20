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
    #[test]
    /// A vector has 3 fields:
    /// * the length,
    /// * the capacity,
    /// * a pointer to a heap allocation where the elements are stored
    fn vec() {
        // accessing elements
        let nums = vec![0, 1, 2, 3, 4];
        let lines = vec![String::from("line one"), String::from("line two")];
        assert_eq!(4, nums[4]);
        let line2 = lines[1].clone(); // requires clone
        let line2_ref = &lines[1];
        assert_eq!(String::from("line two"), line2);
        assert_eq!("line two", line2_ref);

        println!("lines: {:?}", lines);
    }
}
