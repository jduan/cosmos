pub fn run() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // the type is needed because Rust can't infer the type for None
    let absent_number: Option<i32> = None;
}
