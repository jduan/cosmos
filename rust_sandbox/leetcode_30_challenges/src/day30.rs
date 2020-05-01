use std::cell::RefCell;
use std::rc::Rc;
/// Check If a String Is a Valid Sequence from Root to Leaves Path in a Binary Tree
///
/// Given a binary tree where each path going from the root to any leaf form a valid sequence,
/// check if a given string is a valid sequence in such binary tree.
///
/// We get the given string from the concatenation of an array of integers arr and the
/// concatenation of all values of the nodes along a path results in a sequence in the given
/// binary tree.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        Self::recur(&root, &arr, 0)
    }

    // Return the "max sum" starting from this node down, just one path.
    // When we recur, we maintain and update "max_diameter", which is defined as:
    // node.val + left_sum (if left_sum > 0) + right_sum (if right_sum > 0)
    fn recur(node: &Option<Rc<RefCell<TreeNode>>>, arr: &[i32], depth: usize) -> bool {
        if let Some(node) = node {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if left.is_none() && right.is_none() {
                return depth == arr.len() - 1 && node.val == arr[depth];
            }
            if depth >= arr.len() || node.val != arr[depth] {
                return false;
            }
            Self::recur(left, arr, depth + 1) || Self::recur(right, arr, depth + 1)
        } else {
            false
        }
    }
}
