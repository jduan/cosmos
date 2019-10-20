use std::collections::HashMap;

pub fn run() {
    create_maps();
}

fn create_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // use zip and collector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many
    // different data structures and Rust doesn’t know which you want unless you specify. For the
    // parameters for the key and value types, however, we use underscores, and Rust can infer the
    // types that the hash map contains based on the types of the data in the vectors.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
