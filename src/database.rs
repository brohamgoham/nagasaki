use std::io;
use std::fs;
use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tui::{widgets::{ListState}};

/*
 * Database.rs connects to the filesystem and reads the DB file. 
 */

const DB_PATH: &str = "./data/db.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}

pub fn read_data() -> Result<Vec<Planet>, Error> {
    let db_content = fs::read_to_string(DB_PATH)
        .expect("Error trying to read the DB FILE!");

    let parsed: Vec<Planet> = serde_json::from_str(&db_content)
        .expect("Error trying to parse the DB FILE!!!!!");

    OK(parsed)
}

pub fn add_rand() -> Result<Vec<Planet>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)
        .expect("Error trying to read the BD FILE!");

    let mut parsed: Vec<Planet> = serde_json::from_str(&db_content)
        .expect("Cannot parse the DB FILE");

    let orbit = match rng.gen_range(0, 1) {
        0 => "Earth",
        _ => "Jupiter",       
    };

    let rand_people = Planet {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: orbit.to_owned(),
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
    };

    parsed.push(rand_people);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed).unwrap())
        .expect("Error trying to write the DB FILE!");

    Ok(parsed)
}

pub fn remove_planet_index(planet_list: &mut ListState) -> Result<(), Error> {
    if let Some(select) = planet_list.selected() {
        let db_content = fs::read_to_string(DB_PATH)
            .expect("Error trying to read the DB FILE!");

        let mut parsed: Vec<Planet> = serde_json::from_str(&db_content)?;

        if parsed.len() > 0 {
            parsed.remove(select);
            fs::write(DB_PATH, &serde_json::from_vec(&parsed)
               .expect("error trying to write the db file!")
            )?;

            if select == 0 {
                planet_list.select(Some(select));
            } else {
                planet_list.select(Some(select - 1));
            }
        }
    }
    Ok(())
}