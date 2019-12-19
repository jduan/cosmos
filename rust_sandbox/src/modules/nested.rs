pub fn function() {
    println!("called `modules::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `modules::nested::private_function()`");
}

// Functions declared using `pub(in path)` syntax are only visible
// within the given path. `path` must be a parent or ancestor module
pub(in crate::modules) fn public_function_in_modules() {
    print!("called `modules::nested::public_function_in_modules()`, that\n> ");
    public_function_in_nested();
}

// Functions declared using `pub(self)` syntax are only visible within
// the current module, which is the same as leaving them private
pub(self) fn public_function_in_nested() {
    println!("called `modules::nested::public_function_in_nested()`");
}

// Functions declared using `pub(super)` syntax are only visible within
// the parent module
pub(super) fn public_function_in_super_mod() {
    println!("called `modules::nested::public_function_in_super_mod()`");
}
