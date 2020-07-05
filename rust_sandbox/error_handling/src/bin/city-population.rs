/// This is a case study that demonstrates how to do error handling when writing a CLI app.
#[macro_use]
extern crate clap;

use clap::App;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs;

// A row of the CSV file
#[derive(Debug, Deserialize)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,

    // Not every row has these data.
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

struct PopulationCount {
    country: String,
    city: String,
    count: u64,
}

/// Return a vector of PopulationCount for rows that match the city name and have a population
/// column.
fn get_population(data_path: &str, city: &str) -> Result<Vec<PopulationCount>, Box<dyn Error>> {
    let mut found = vec![];
    let file = fs::File::open(data_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    for (_line, row) in reader.deserialize().enumerate() {
        let row: Row = row?;

        if row.city == city {
            if let Some(count) = row.population {
                found.push(PopulationCount {
                    country: row.country,
                    city: row.city,
                    count,
                })
            }
        }
    }

    if found.is_empty() {
        Err(From::from(
            "No matching cities with a population were found.",
        ))
    } else {
        Ok(found)
    }
}

fn main() {
    // Relative to the current file
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // We can safely call "unwrap" here because these 2 are "required" arguments.
    let data_path = m.value_of("data_path").unwrap();
    let city = m.value_of("city").unwrap();

    match get_population(data_path, city) {
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
