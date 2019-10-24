mod control_flow;
mod enums;
mod exercise;
mod functions;
mod guess_my_number;
mod hash_maps;
mod if_let;
mod method_syntax;
mod options;
mod ownership;
mod pattern_matching;
mod slice_type;
mod strings;
mod structs;
mod variables;
mod vectors;
mod recover_from_errors;
mod generics;

fn main() {
    // guess_my_number::run();
    // variables::run()
    // println!("10 + 5 is {}", functions::add5(10));
    // control_flow::run();
    // exercise::run();
    // ownership::run();
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
    generics::run()
}

fn will_panic() {
    let v = vec![1, 2, 3];
    v[99];
}
