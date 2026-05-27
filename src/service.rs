// define types to serde
// make requests

use anyhow;
use serde::{Deserialize, Serialize};

use crate::cli;

/*
{
    "name": {
        "common": "Gambia",
        "official": "Republic of the Gambia",
        "nativeName": {
        "eng": {
            "official": "Republic of the Gambia",
            "common": "Gambia"
        }
        }
    },
    "region": "Africa",
    "area": 10689,
    "population": 2422712
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: Name,
    region: String,
    area: f32,
    population: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    common: String,
    official: String,
    // nativeName
}

// question: is my return type OK?
pub fn get_results(args: cli::Args) -> anyhow::Result<Vec<Item>> {
    let body = reqwest::blocking::get(
        "https://restcountries.com/v3.1/all?fields=name,region,population,area",
    )?;

    let x = body.json::<Vec<Item>>()?;
    Ok(x)
}
