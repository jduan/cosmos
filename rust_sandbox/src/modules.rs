// Items in modules default to private visibility.
fn private_function() {
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

// Modules can also be nested
pub mod nested {
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

// Nested modules follow the same rules for visibility
mod private_nested {
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
