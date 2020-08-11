/// Last Stone Weight
///
/// We have a collection of stones, each stone has a positive integer weight.
///
/// Each turn, we choose the two heaviest stones and smash them together.  Suppose the stones have
/// weights x and y with x <= y.  The result of this smash is:
///
///     If x == y, both stones are totally destroyed;
///     If x != y, the stone of weight x is totally destroyed, and the stone of weight y has new weight y-x.
///
/// At the end, there is at most 1 stone left.  Return the weight of this stone (or 0 if there are no stones left.)
///
/// Example 1:
///
/// Input: [2,7,4,1,8,1]
/// Output: 1
/// Explanation:
/// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
/// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
/// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
/// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        for weight in stones {
            heap.push(weight);
        }

        loop {
            match heap.len() {
                0 => return 0,
                1 => return heap.pop().unwrap(),
                _ => {
                    let w1 = heap.pop().unwrap();
                    let w2 = heap.pop().unwrap();
                    let diff = w1 - w2;
                    if diff > 0 {
                        heap.push(diff);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }
}
