#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // Relative to the current file
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    match m.value_of("config") {
        None => println!("not config file passed"),
        Some(config) => println!("config file: {}", config),
    }

    let input = m.value_of("INPUT").unwrap();
    println!("INPUT file is {}", input);

    match m.occurrences_of("verbose") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }
}
