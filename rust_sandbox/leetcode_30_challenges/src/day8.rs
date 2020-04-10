///  Middle of the Linked List
///
/// Given a non-empty, singly linked list with head node head, return a middle node of linked list.
///
/// If there are two middle nodes, return the second middle node.
///
///  
///
/// Example 1:
///
/// Input: [1,2,3,4,5]
/// Output: Node 3 from this list (Serialization: [3,4,5])
/// The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
/// Note that we returned a ListNode object ans, such that:
/// ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
///
/// Example 2:
///
/// Input: [1,2,3,4,5,6]
/// Output: Node 4 from this list (Serialization: [4,5,6])
/// Since the list has two middle nodes with values 3 and 4, we return the second one.
///
///  
///
/// Note:
///
///     The number of nodes in the given list will be between 1 and 100.

#[allow(dead_code)]
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    fn append(val: i32, other: ListNode) -> Self {
        ListNode {
            next: Some(Box::new(other)),
            val,
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(head) => {
                let mut slow = &head;
                let mut fast = &head;
                loop {
                    let fast_next = fast.next.as_ref();
                    if let Some(fast_next) = fast_next {
                        let fast_next_next = fast_next.next.as_ref();
                        if let Some(fast_next_next) = fast_next_next {
                            fast = &fast_next_next;
                            slow = &slow.next.as_ref().unwrap();
                        } else {
                            slow = &slow.next.as_ref().unwrap();
                            return Some(slow.to_owned());
                        }
                    } else {
                        return Some(slow.to_owned());
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
    fn test_hello1() {
        // original list
        let node6 = ListNode::new(6);
        let node5 = ListNode::append(5, node6);
        let node4 = ListNode::append(4, node5);
        // copy of the 2nd half before the ownership of node4 gets moved
        let middle_node_copy = node4.clone();
        let node3 = ListNode::append(3, node4);
        let node2 = ListNode::append(2, node3);
        let node1 = ListNode::append(1, node2);

        let middle_node = Solution::middle_node(Some(Box::new(node1)));

        assert_eq!(Box::new(middle_node_copy), middle_node.unwrap());
    }

    #[test]
    fn test_hello2() {
        // original list
        let node5 = ListNode::new(5);
        let node4 = ListNode::append(4, node5);
        let node3 = ListNode::append(3, node4);
        // copy of the 2nd half before the ownership of node3 gets moved
        let middle_node_copy = node3.clone();
        let node2 = ListNode::append(2, node3);
        let node1 = ListNode::append(1, node2);

        let middle_node = Solution::middle_node(Some(Box::new(node1)));

        assert_eq!(Box::new(middle_node_copy), middle_node.unwrap());
    }
}
