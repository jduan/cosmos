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

#[cfg(test)]
mod tests {
    use crate::enums::{inspect, WebEvent};

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
}
