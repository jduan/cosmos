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
}
