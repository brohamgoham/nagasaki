use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use thiserror::Error;
use tui::widgets::ListState;

/*
 * Database.rs connects to the filesystem and reads the DB file.
 */

const DB_PATH: &str = "./data/db.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Planet {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}

/// Read data from DB, parse as JSON and return wrapped value
pub fn read_data() -> Result<Vec<Planet>, Error> {
    // Read contents of file
    let db_content = fs::read_to_string(DB_PATH).expect("Error trying to read the DB FILE!");

    let parsed: Vec<Planet> =
        serde_json::from_str(&db_content).expect("Error trying to parse the DB FILE!!!!!");

    Ok(parsed)
}

/// Generate a random planet and add it to DB
/// add to a Vec<Planet> and write to DB
/// returns the updated Vec<Planet>
pub fn add_rand() -> Result<Vec<Planet>, Error> {
    // create random num generator
    let mut rng = rand::thread_rng();

    // read the entire file contents
    let db_content = fs::read_to_string(DB_PATH).expect("Error trying to read the BD FILE!");

    // serde_json::from_str for parsing db_content string
    // as JSON encoded, returns error if cannot parse
    let mut parsed: Vec<Planet> =
        serde_json::from_str(&db_content).expect("Cannot parse the DB FILE");

    // generate random orbit using match 
    let orbit = match rng.gen_range(0, 9) {
        0 => "Earth",
        1 => "Jupiter",
        2 => "Mars",
        3 => "Mercury",
        4 => "Neptune",
        5 => "Pluto",
        6 => "Saturn",
        7 => "Uranus",
        8 => "Venus",
        _ => "Sun",
    };

    // create a new Planet struct with random values
    let rand_planet = Planet {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: orbit.to_owned(),
        age: rng.gen_range(1000000000, 150000000000),
        created_at: Utc::now(),
    };

    // Add new planet to the parsed Vec<Planet>
    parsed.push(rand_planet);
    // serde_json::to_vec is used to serialize the parsed Vec to a JSON encoded Vec
    fs::write(DB_PATH, &serde_json::to_vec(&parsed).unwrap())
        .expect("Error trying to write the DB FILE!");

    Ok(parsed)
}

/// Remove a planet from DB
/// takes mutable reference to ListState, and removes the selected planet
pub fn remove_planet_index(planet_list: &mut ListState) -> Result<(), Error> {
    // using match expression to check if there is currently a selected planet in the list
    if let Some(select) = planet_list.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<Planet> = serde_json::from_str(&db_content)?;

        // check ig parsed vector is not empty
        if parsed.len() > 0 {
            // rm selected planet at selected index from the parsed Vec
            parsed.remove(select);
            fs::write(
                DB_PATH,
                &serde_json::to_vec(&parsed).expect("error trying to write the db file!"),
            )?;

            // update the selected index of planet_list to previous index
            if select == 0 {
                planet_list.select(Some(select));
            } else {
                planet_list.select(Some(select - 1));
            }
        }
    }
    Ok(())
}
