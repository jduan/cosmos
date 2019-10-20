use std::collections::HashMap;

pub fn run() {
    create_maps();
    ownership();
    access_values();
    iterate_values();
    update_map();
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

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are owned by the hash map now!
    // the line below won't compile
    // println!("field_name is {}", field_name);
}

fn access_values() {
    let scores = get_map();

    let score = scores.get(&String::from("Blue"));
    match score {
        Some(v) => println!("Blue's score is {}", v),
        None => println!("Can't find Blue teams's score"),
    }
}

fn iterate_values() {
    let scores = get_map();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn update_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // this overrides the previous value since the key is the same
    scores.insert(String::from("Blue"), 50);
    println!("The map is {:?}", scores);

    // only insert if the key doesn't exist
    // the return value of the "entry" method is an enum called Entry that
    // represents a value that may or may not exist.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(10);
    println!("The map is {:?}", scores);

    // update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count is a mutable reference to the value
        *count += 1;
    }
    // the mutable references of "count" go out of scope at the end of the for loop,
    // which is why we can access "map" here.
    println!("word count: {:?}", map);
}

// create a example map
fn get_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores
}
