// define types to serde
// make requests

use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

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
    languages: Option<HashMap<String, String>>,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // as_ref converts Option<HashMap> to Option<&HashMap>
        // first map changes content in Option
        // second map goes over hashmap and converts to one string
        let languages = self
            .languages
            .as_ref()
            .map(|hashmap| {
                hashmap
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or("none".to_string());
        write!(
            f,
            "[ name={} | region=\"{}\" | area={} | population={} | languages=\"{}\" ]",
            self.name, self.region, self.area, self.population, languages,
        )
    }
}

// impl Display for Vec<Item> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(f, "[{}]", self.item)
//     }
// }

// question: why do I need Eq, Ord, PartialEq and PartialOrd
#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Name {
    common: String,
    official: String,
    // nativeName
}
impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "[ common=\"{}\" | official=\"{}\" ]",
            self.common, self.official
        )
    }
}

// question: is my return type OK?
pub fn get_results(args: cli::Args) -> anyhow::Result<Vec<Item>> {
    let body = reqwest::blocking::get(
        "https://restcountries.com/v3.1/all?fields=name,region,population,area,languages",
    )?;

    let xs = body.json::<Vec<Item>>()?;

    // question: difference between iter() and into_iter()
    // question: is this really more readable
    let mut xs: Vec<_> = xs
        .into_iter()
        .filter(|x| {
            args.region
                .as_deref()
                .map_or(true, |region| region == x.region)
        })
        .filter(|x| {
            args.language.as_deref().map_or(true, |lang_arg| {
                x.languages.as_ref().map_or(false, |hashmap| {
                    hashmap.values().any(|lang| lang == lang_arg)
                })
            })
        })
        .collect();

    // sort order desc by default (except names)
    match args.sort {
        cli::SortBy::Population => xs.sort_by_key(|x| Reverse(x.population)),
        cli::SortBy::Area => xs.sort_by(|a, b| b.area.total_cmp(&a.area)),
        cli::SortBy::Name => xs.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    let xs = xs.into_iter().take(args.top).collect();

    Ok(xs)
}
