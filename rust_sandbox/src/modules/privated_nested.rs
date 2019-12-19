#[allow(dead_code)]
pub fn function() {
    println!("called `modules::private_nested::function()`");
}

// Private parent items will still restrict the visibility of a child item,
// even if it is declared as visible within a bigger scope.
#[allow(dead_code)]
pub(crate) fn restricted_function() {
    println!("called `modules::private_nested::restricted_function()`");
}
