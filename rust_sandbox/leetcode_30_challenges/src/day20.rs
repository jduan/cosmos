use std::cell::RefCell;
use std::rc::Rc;
/// Construct Binary Search Tree from Preorder Traversal
///
/// Return the root node of a binary search tree that matches the given preorder traversal.
///
/// (Recall that a binary search tree is a binary tree where for every node, any descendant of
/// node.left has a value < node.val, and any descendant of node.right has a value > node.val.
/// Also recall that a preorder traversal displays the value of the node first, then traverses
/// node.left, then traverses node.right.)
///
///
///
/// Example 1:
///
/// Input: [8,5,1,7,10,12]
/// Output: [8,5,10,1,7,null,12]
///
///                    +---+
///                    | 8 |
///             +------+---+-----+
///             |                |
///          +--v+            +--v-+
///          | 5 |            | 10 |
///    +-----+---+--+         +----+---+
///    |            |                  |
///    |            |                  |
/// +--v-+       +--v--+            +--v--+
/// | 1  |       |  7  |            |  12 |
/// +----+       +-----+            +-----+
///
///
/// Note:
///
///     1 <= preorder.length <= 100
///     The values of preorder are distinct.

// Definition for a binary tree node.
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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recur(&preorder)
    }

    fn recur(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
            _ => {
                let root = preorder[0];
                // Find the first element that's greater than root
                let mut i = 0;
                while i < preorder.len() {
                    if preorder[i] > root {
                        break;
                    }
                    i += 1;
                }
                let mut root_node = TreeNode::new(root);
                let (left_node, right_node) = if i >= preorder.len() {
                    // none of the nodes is greater than the first node
                    (Self::recur(&preorder[1..]), None)
                } else if i == 1 {
                    (None, Self::recur(&preorder[1..]))
                } else {
                    (Self::recur(&preorder[1..i]), Self::recur(&preorder[i..]))
                };

                root_node.left = left_node;
                root_node.right = right_node;

                Some(Rc::new(RefCell::new(root_node)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_from_preorder() {
        let root = Solution::bst_from_preorder(vec![]);
        assert!(root.is_none());

        let root = Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]);
        assert!(root.is_some());
        let root_node = root.unwrap();
        let root_node = root_node.borrow_mut();
        assert_eq!(8, root_node.val);
    }
}
