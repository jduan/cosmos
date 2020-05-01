/// You are given two non-empty linked lists representing two non-negative integers. The digits
/// are stored in reverse order and each of their nodes contain a single digit. Add the two numbers
/// and return it as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// Example:
///
/// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
/// Output: 7 -> 0 -> 8
/// Explanation: 342 + 465 = 807.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    #[inline]
    fn new2(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::recur(l1, l2, 0)
    }

    fn recur(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry_over: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), Some(node2)) => {
                let sum = node1.val + node2.val + carry_over;
                let (val, carry_over) = if sum > 9 { (sum - 10, 1) } else { (sum, 0) };
                let mut node = ListNode::new(val);
                node.next = Self::recur(node1.next, node2.next, carry_over);

                Some(Box::new(node))
            }
            (None, None) => {
                if carry_over > 0 {
                    Some(Box::new(ListNode::new(carry_over)))
                } else {
                    None
                }
            }
            (Some(node1), None) => Self::add_one_number(Some(node1), carry_over),
            (None, Some(node2)) => Self::add_one_number(Some(node2), carry_over),
        }
    }

    fn add_one_number(list: Option<Box<ListNode>>, num: i32) -> Option<Box<ListNode>> {
        if num == 0 {
            return list;
        }

        if let Some(node) = list {
            let sum = node.val + num;
            let (val, carry_over) = if sum > 9 { (sum - 10, 1) } else { (sum, 0) };
            let mut new_node = ListNode::new(val);
            new_node.next = Self::add_one_number(node.next, carry_over);

            Some(Box::new(new_node))
        } else {
            Some(Box::new(ListNode::new(num)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        // 342 + 465 = 807
        let node3 = ListNode::new2(3, None);
        let node4 = ListNode::new2(4, Some(Box::new(node3)));
        let node2 = ListNode::new2(2, Some(Box::new(node4)));
        let list1 = Some(Box::new(node2));

        let node4 = ListNode::new2(4, None);
        let node6 = ListNode::new2(6, Some(Box::new(node4)));
        let node5 = ListNode::new2(5, Some(Box::new(node6)));
        let list2 = Some(Box::new(node5));

        let node8 = ListNode::new2(8, None);
        let node0 = ListNode::new2(0, Some(Box::new(node8)));
        let node7 = ListNode::new2(7, Some(Box::new(node0)));
        let expected_list = Some(Box::new(node7));

        assert_eq!(expected_list, Solution::add_two_numbers(list1, list2));
    }

    #[test]
    fn test_add_two_numbers2() {
        // 99 + 99 = 198
        let node1 = ListNode::new2(9, None);
        let node2 = ListNode::new2(9, Some(Box::new(node1)));
        let list1 = Some(Box::new(node2));

        let node1 = ListNode::new2(9, None);
        let node2 = ListNode::new2(9, Some(Box::new(node1)));
        let list2 = Some(Box::new(node2));

        let node1 = ListNode::new2(1, None);
        let node2 = ListNode::new2(9, Some(Box::new(node1)));
        let node3 = ListNode::new2(8, Some(Box::new(node2)));
        let expected_list = Some(Box::new(node3));

        assert_eq!(expected_list, Solution::add_two_numbers(list1, list2));
    }

    #[test]
    fn test_add_two_numbers3() {
        // one of the lists is empty
        let node1 = ListNode::new2(9, None);
        let node2 = ListNode::new2(9, Some(Box::new(node1)));
        let list1 = Some(Box::new(node2));

        let expected_list = list1.clone();
        assert_eq!(expected_list, Solution::add_two_numbers(list1, None));
    }
}
