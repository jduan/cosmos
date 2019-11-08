// This declaration will look for a file named "foo.rs" or "foo/mod.rs" and
// insert its contents inside a module named `foo` under this scope. Hence
// you can call functions like `foo::answer()` in this file.
mod foo;

mod closures;
mod control_flow;
mod enums;
mod exercise;
mod functions;
mod generics;
mod guess_my_number;
mod hash_maps;
mod if_let;
mod iterators;
mod lifetime;
mod method_syntax;
mod options;
mod ownership;
mod pattern_matching;
mod recover_from_errors;
mod references;
mod slice_type;
mod smart_pointers;
mod strings;
mod structs;
mod threads;
mod traits;
mod variables;
mod vectors;

use foo::answer;

fn main() {
    // println!("foo::answer: {}", foo::answer());
    // println!("foo::answer: {}", answer());
    // println!("foo::bar::question: {}", foo::bar::question());
    // guess_my_number::run();
    // variables::run()
    // println!("10 + 5 is {}", functions::add5(10));
    // control_flow::run();
    // exercise::run();
    // ownership::run();
    references::run();
    // slice_type::run();
    // structs::run();
    // method_syntax::run();
    // enums::run();
    // options::run();
    // pattern_matching::run();
    // if_let::run();
    // vectors::run();
    // strings::run();
    // hash_maps::run();
    // will_panic();
    // recover_from_errors::run();
    // generics::run();
    // traits::run();
    // lifetime::run();
    // closures::run();
    // iterators::run();
    // smart_pointers::run();
    // threads::run();
}

fn will_panic() {
    let v = vec![1, 2, 3];
    v[99];
}
