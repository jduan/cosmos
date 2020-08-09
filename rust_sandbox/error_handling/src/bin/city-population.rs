/// This is a case study that demonstrates how to do error handling when writing a CLI app.
#[macro_use]
extern crate clap;

use clap::App;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::fmt::Display;
use std::fs;
use std::io;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Csv(csv::Error),
    NotFound,
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(err) => err.fmt(f),
            CliError::Csv(err) => err.fmt(f),
            CliError::NotFound => write!(f, "No matching cities with a population were found."),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        CliError::Csv(err)
    }
}

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
fn get_population(data_path: Option<&str>, city: &str) -> Result<Vec<PopulationCount>, CliError> {
    let mut found = vec![];
    let input: Box<dyn io::Read> = match data_path {
        None => Box::new(io::stdin()),
        Some(file_path) => Box::new(fs::File::open(file_path)?),
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(input);
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
        Err(CliError::NotFound)
    } else {
        Ok(found)
    }
}

fn main() {
    // Relative to the current file
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // We can safely call "unwrap" here because these 2 are "required" arguments.
    let data_path = m.value_of("data_path");
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
