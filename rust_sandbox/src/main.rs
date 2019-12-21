#![feature(never_type)]

// declare modules
pub mod arrays;
pub mod associate_types;
pub mod casting;
pub mod clone_trait;
pub mod closures;
pub mod constants;
pub mod control_flow;
pub mod conversions;
pub mod debug_trait;
pub mod diverging_functions;
pub mod enums;
pub mod exercise;
pub mod expressions;
pub mod foo;
pub mod functions;
pub mod generics;
pub mod guess_my_number;
pub mod hash_maps;
pub mod if_let;
pub mod interior_mutability;
pub mod iterators;
pub mod lifetime;
pub mod literals;
pub mod method_syntax;
pub mod misc;
pub mod modules;
pub mod options;
pub mod ownership;
pub mod pattern_matching;
pub mod phantom_types;
pub mod print_things;
pub mod recover_from_errors;
pub mod references;
pub mod sandbox;
pub mod slice_type;
pub mod smart_pointers;
pub mod strings;
pub mod structs;
pub mod supertrait;
pub mod threads;
pub mod traits;
pub mod type_alias;
pub mod variables;
pub mod vectors;
pub mod while_let;

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
    // references::run();
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
    sandbox::run();
}

fn will_panic() {
    let v = vec![1, 2, 3];
    v[99];
}
