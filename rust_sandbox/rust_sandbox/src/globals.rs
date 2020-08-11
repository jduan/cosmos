// https://github.com/paulkernfeld/global-data-in-rust

// This constant can be used by other modules.
pub const MY_NAME: &str = "paul";

// The presence of the file is checked at compile time!
pub const SAMPLE_STR: &str = include_str!("./data/sample.txt");
