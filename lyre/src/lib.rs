use std::{error::Error, fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Series {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetaData {
    pub title: String,
    pub slug: String,
    pub tagline: Option<String>,
    pub series: Option<Series>,
    pub tags: Vec<String>,
}

impl MetaData {
    pub fn open<P>(path: P) -> Result<Self, impl Error>
    where
        P: AsRef<Path>,
    {
        serde_yaml::from_reader(File::open(path).unwrap())
    }
}
