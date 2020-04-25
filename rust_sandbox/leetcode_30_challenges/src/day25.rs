use std::cmp::min;
use std::collections::{HashSet, VecDeque};

///  Jump Game
///
/// Given an array of non-negative integers, you are initially positioned at the first index of the
/// array.
///
/// Each element in the array represents your maximum jump length at that position.
///
/// Determine if you are able to reach the last index.
///
/// Example 1:
///
/// Input: [2,3,1,1,4]
/// Output: true
/// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
///
/// Example 2:
///
/// Input: [3,2,1,0,4]
/// Output: false
/// Explanation: You will always arrive at index 3 no matter what. Its maximum
///              jump length is 0, which makes it impossible to reach the last index.

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(num: Vec<i32>) -> bool {
        let num2 = num.iter().map(|n| *n as usize).collect();
        // Self::can_jump_new(num2)
        Self::dynamic_programming(num2)
    }

    /// We call a position in the array a "good index" if starting at that position, we can reach
    /// the last index. Otherwise, that index is called a "bad index".
    /// The idea is that we iterate from the end to the beginning. For each position,
    /// we check if it can jump to a position that is a good index and we mark accordingly.
    /// Once we are at index 0, we are done.
    fn dynamic_programming(num: Vec<usize>) -> bool {
        let mut memory: Vec<bool> = Vec::with_capacity(num.len());
        let len = num.len();
        for _i in 0..len {
            memory.push(false);
        }
        memory[len - 1] = true;
        for i in (0..(len - 1)).rev() {
            let jumps = num[i];
            let end = min(i + jumps, len - 1);
            for jump in i..=end {
                if memory[jump] {
                    memory[i] = true;
                    break;
                }
            }
        }

        memory[0]
    }

    // It's much easier to work with a vector of usize
    fn can_jump_new(num: Vec<usize>) -> bool {
        if num.is_empty() {
            return false;
        }
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut visited = HashSet::new();
        visited.insert(0);

        Self::bfs(num, queue, visited)
    }

    fn bfs(num: Vec<usize>, mut queue: VecDeque<usize>, mut visited: HashSet<usize>) -> bool {
        while !queue.is_empty() {
            let idx = queue.pop_front().unwrap();
            let jumps = num[idx];
            for i in (0..=jumps).rev() {
                let next_idx = idx + i;
                if next_idx == num.len() - 1 {
                    return true;
                }
                if next_idx < num.len() && !visited.contains(&next_idx) {
                    visited.insert(next_idx);
                    queue.push_back(next_idx);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(true, Solution::can_jump(vec![0]));
        assert_eq!(true, Solution::can_jump(vec![1]));
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));

        let num: Vec<i32> = (1000..25000).rev().collect();
        assert_eq!(true, Solution::can_jump(num));
    }
}
