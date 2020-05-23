#![feature(never_type)]
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref GLOBAL_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("city", "Orinda");
        m
    };
}

// use crate::command_example::run_command;
use crate::globals::MY_NAME;
use crate::globals::SAMPLE_STR;

// declare modules
pub mod arrays;
pub mod associate_types;
pub mod async_example;
pub mod casting;
pub mod channels;
pub mod clone_trait;
pub mod closures;
pub mod collections;
pub mod command_example;
pub mod constants;
pub mod control_flow;
pub mod conversions;
pub mod debug_trait;
pub mod diverging_functions;
pub mod enums;
pub mod error_handling;
pub mod error_handling2;
pub mod exercise;
pub mod expressions;
pub mod foo;
pub mod functions;
pub mod generics;
pub mod globals;
pub mod guess_my_number;
pub mod hash_maps;
pub mod if_let;
pub mod input_output;
pub mod interior_mutability;
pub mod iterators;
pub mod lifetime;
pub mod literals;
pub mod macros;
pub mod map_reduce;
pub mod method_syntax;
pub mod misc;
pub mod modules;
pub mod options;
pub mod ownership;
pub mod pattern_matching;
pub mod phantom_types;
pub mod print_things;
pub mod processes;
pub mod references;
pub mod sandbox;
pub mod serde_examples;
pub mod slice_type;
pub mod smart_pointers;
pub mod strings;
pub mod structs;
pub mod supertrait;
pub mod template;
pub mod threads;
pub mod traits;
pub mod type_alias;
pub mod variables;
pub mod vectors;
pub mod while_let;

pub fn main() {
    println!("Hello World from Rust!");
    // run_command().unwrap();
    println!("My name is {}", MY_NAME);
    println!("Sample txt is {}", SAMPLE_STR);
    println!("The city is {:?}", GLOBAL_MAP.get(&"city"));
    assert_eq!(MY_NAME, "paul");
}
