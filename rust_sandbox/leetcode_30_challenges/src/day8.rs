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
    fn test_hello() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let node3 = ListNode::new(3);
        let node2_clone = node2.clone();
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let middle_node = Solution::middle_node(Some(Box::new(node1)));

        assert_eq!(node2_clone.val, middle_node.unwrap().val);
    }
}
