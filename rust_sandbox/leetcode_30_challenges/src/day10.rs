///   Min Stack
///
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
///
///     push(x) -- Push element x onto stack.
///     pop() -- Removes the element on top of the stack.
///     top() -- Get the top element.
///     getMin() -- Retrieve the minimum element in the stack.
///
///  
///
/// Example:
///
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin();   --> Returns -3.
/// minStack.pop();
/// minStack.top();      --> Returns 0.
/// minStack.getMin();   --> Returns -2.

#[cfg(test)]
struct Pair {
    element: i32,
    current_min: i32,
}

#[cfg(test)]
pub struct MinStack {
    elements: Vec<Pair>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[cfg(test)]
impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        if self.elements.is_empty() {
            self.elements.push(Pair {
                element: x,
                current_min: x,
            });
        } else {
            let current_min = self.elements.last().unwrap().current_min;
            let pair = if x < current_min {
                Pair {
                    element: x,
                    current_min: x,
                }
            } else {
                Pair {
                    element: x,
                    current_min,
                }
            };
            self.elements.push(pair)
        }
    }

    pub fn pop(&mut self) {
        if !self.elements.is_empty() {
            self.elements.pop();
        }
    }

    pub fn top(&self) -> i32 {
        self.elements.last().unwrap().element
    }

    pub fn get_min(&self) -> i32 {
        self.elements.last().unwrap().current_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(-3, stack.get_min());
        stack.pop();
        assert_eq!(0, stack.top());
        assert_eq!(-2, stack.get_min());
    }
}
