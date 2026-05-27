// define types to serde
// make requests

use std::cmp::Reverse;

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

// question: why do I need Eq, Ord, PartialEq and PartialOrd
#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

    let xs = body.json::<Vec<Item>>()?;

    // question: difference between iter() and into_iter()
    // question: is this really more readable
    let mut xs: Vec<_> = xs
        .into_iter()
        .filter(|x| {
            if let Some(ref region) = args.region {
                x.region == *region
            } else {
                true
            }
        })
        .collect();

    match args.sort {
        cli::SortBy::Population => xs.sort_by_key(|x| Reverse(x.population)),
        cli::SortBy::Area => xs.sort_by(|a, b| b.area.total_cmp(&a.area)),
        cli::SortBy::Name => xs.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    let xs = xs.into_iter().take(args.top).collect();

    Ok(xs)
}
