// declare sub modules
mod nested;
mod privated_nested;

// Items in modules default to private visibility.
pub fn private_function() {
    println!("called `modules::private_function()`");
}

// Use the `pub` modifier to override default visibility.
pub fn function() {
    println!("called `modules::function()`");
}

// Items can access other items in the same module,
// even when private.
pub fn indirect_access() {
    print!("called `modules::indirect_access()`, that\n> ");
    private_function();
}

pub fn call_public_function_in_modules() {
    print!("called `modules::call_public_function_in_modules()`, that\n> ");
    nested::public_function_in_modules();
    print!("> ");
    nested::public_function_in_super_mod();
}

// pub(crate) makes functions visible only within the current crate
pub(crate) fn public_function_in_crate() {
    println!("called `modules::public_function_in_crate()`");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules() {
        function();

        // Public items, including those inside nested modules, can be
        // accessed from outside the parent module.
        indirect_access();
        nested::function();
        call_public_function_in_modules();

        // pub(crate) items can be called from anywhere in the same crate
        public_function_in_crate();

        // pub(in path) items can only be called from within the mode specified
        // Error! function `public_function_in_modules` is private
        //nested::public_function_in_modules();
        // TODO ^ Try uncommenting this line

        // Private items of a module cannot be directly accessed, even if
        // nested in a public module:

        // Error! `private_function` is private
        //private_function();
        // TODO ^ Try uncommenting this line

        // Error! `private_function` is private
        //nested::private_function();
        // TODO ^ Try uncommenting this line

        // Error! `private_nested` is a private module
        //private_nested::function();
        // TODO ^ Try uncommenting this line

        // Error! `private_nested` is a private module
        //private_nested::restricted_function();
        // TODO ^ Try uncommenting this line
    }
}
