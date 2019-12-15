use crate::enums::List::{Cons, Nil};

/// The enum keyword allows the creation of a type which may be one of a few different variants.
/// Any variant which is valid as a struct is also valid as an enum.

pub enum WebEvent {
    // unit like
    PageLoad,
    PageUnload,

    // like tuple structs
    KeyPress(char),
    Paste(String),

    // C-like structs
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) -> String {
    match event {
        WebEvent::PageLoad => String::from("page loaded"),
        WebEvent::PageUnload => String::from("page unloaded"),
        WebEvent::KeyPress(c) => format!("pressed {}", c),
        WebEvent::Paste(s) => format!("pasted '{}'", s),
        WebEvent::Click { x, y } => format!("clicked at x={}, y={}", x, y),
    }
}

#[derive(PartialEq)]
pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl Default for List {
    fn default() -> Self {
        Nil
    }
}

impl List {
    // Create an empty List
    pub fn new() -> List {
        Nil
    }

    pub fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match self {
            Nil => 0,
            Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn stringify(&self) -> String {
        fn helper(lst: &List) -> String {
            match lst {
                Nil => String::from(""),
                Cons(head, tail) => {
                    if tail.as_ref() == &Nil {
                        format!("{}", head)
                    } else {
                        format!("{}, {}", head, helper(tail))
                    }
                }
            }
        }

        format!("[{}]", helper(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::{inspect, List, WebEvent};

    #[test]
    fn test_inspect() {
        assert_eq!("page loaded", inspect(WebEvent::PageLoad));
        assert_eq!("page unloaded", inspect(WebEvent::PageUnload));
        assert_eq!("pressed x", inspect(WebEvent::KeyPress('x')));
        assert_eq!(
            "pasted 'hello'",
            inspect(WebEvent::Paste(String::from("hello")))
        );
        assert_eq!(
            "clicked at x=3, y=4",
            inspect(WebEvent::Click { x: 3, y: 4 })
        );
    }

    #[test]
    fn test_list() {
        let lst = List::new();
        assert_eq!(0, lst.len());
        assert_eq!("[]", &lst.stringify());

        let lst = lst.prepend(1);
        assert_eq!(1, lst.len());
        let lst = lst.prepend(2);
        assert_eq!(2, lst.len());

        assert_eq!("[2, 1]", &lst.stringify());
    }
}
