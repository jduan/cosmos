use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
/// Binary Tree Maximum Path Sum
///
/// Given a non-empty binary tree, find the maximum path sum.
///
/// For this problem, a path is defined as any sequence of nodes from some starting node to any
/// node in the tree along the parent-child connections. The path must contain at least one
/// node and does not need to go through the root.
///
/// Example 1:
///
/// Input: [1,2,3]
///
///        1
///       / \
///      2   3
///
/// Output: 6
///
/// Example 2:
///
/// Input: [-10,9,20,null,null,15,7]
///
///    -10
///    / \
///   9  20
///     /  \
///    15   7
///
/// Output: 42

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = std::i32::MIN;
        Solution::recur(&root, &mut max_diameter);

        max_diameter
    }

    // Return the "max sum" starting from this node down, just one path.
    // When we recur, we maintain and update "max_diameter", which is defined as:
    // node.val + left_sum (if left_sum > 0) + right_sum (if right_sum > 0)
    fn recur(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();

            let left_sum = Solution::recur(&node.left, max_diameter);
            let right_sum = Solution::recur(&node.right, max_diameter);
            let mut diameter = node.val;
            if left_sum > 0 {
                diameter += left_sum;
            }
            if right_sum > 0 {
                diameter += right_sum;
            }
            if diameter > *max_diameter {
                *max_diameter = diameter;
            }
            max(0, max(left_sum, right_sum)) + node.val
        } else {
            0
        }
    }
}
