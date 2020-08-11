/// This is a library for working with linked lists.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

#[allow(dead_code)]
impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode { val, next: None }
    }

    pub fn new2(val: T, next: Option<Box<ListNode<T>>>) -> Self {
        ListNode { next, val }
    }

    // Build a linked list from a vector of T.
    pub fn build(mut values: Vec<T>) -> Option<Box<ListNode<T>>> {
        if values.is_empty() {
            None
        } else {
            let head = values.remove(0);
            let mut node = ListNode::new(head);
            node.next = Self::build(values);

            Some(Box::new(node))
        }
    }

    pub fn len(self) -> usize {
        if let Some(next) = self.next {
            1 + next.len()
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let head = ListNode::build(vec![1, 2, 3, 4]);
        assert_eq!(4, head.unwrap().len());

        let v: Vec<i32> = vec![];
        let empty_list = ListNode::build(v);
        assert!(empty_list.is_none());
    }
}
