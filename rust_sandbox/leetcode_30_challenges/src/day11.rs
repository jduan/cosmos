///  Diameter of Binary Tree
///
/// Given a binary tree, you need to compute the length of the diameter of the tree. The diameter
/// of a binary tree is the length of the longest path between any two nodes in a tree. This path
/// may or may not pass through the root.
///
/// Example:
/// Given a binary tree
///
///           1
///          / \
///         2   3
///        / \     
///       4   5    
///
/// Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
///
/// Note: The length of path between two nodes is represented by the number of edges between them.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[cfg(test)]
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
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Solution::recur(&root, &mut max_diameter);

        max_diameter
    }

    // Return the height of the subtree
    fn recur(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        use std::cmp::max;
        if let Some(node) = node {
            let node = node.borrow();

            let left_height = if node.left.is_some() {
                Solution::recur(&node.left, max_diameter) + 1
            } else {
                1
            };
            let right_height = if node.right.is_some() {
                Solution::recur(&node.right, max_diameter) + 1
            } else {
                1
            };
            let current_diameter = left_height + right_height - 2;
            if current_diameter > *max_diameter {
                *max_diameter = current_diameter;
            }

            max(left_height, right_height)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);
        node2.left = Some(Rc::new(RefCell::new(node4)));
        node2.right = Some(Rc::new(RefCell::new(node5)));
        root.left = Some(Rc::new(RefCell::new(node2)));
        root.right = Some(Rc::new(RefCell::new(node3)));

        assert_eq!(
            3,
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))))
        );
    }
}
