use std::{error::Error, fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub brief: Option<String>,
    pub tagline: Option<String>,
    pub series: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub published: String,
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Series {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetaData {
    pub title: String,
    pub slug: String,
    pub brief: String,
    pub tagline: Option<String>,
    pub series: Option<Series>,
    pub tags: Vec<String>,
    pub word_count: usize,
    pub reading_time: usize,
    pub published: String,
    pub updated: Option<String>,
}

impl MetaData {
    pub fn open<P>(path: P) -> Result<Self, impl Error>
    where
        P: AsRef<Path>,
    {
        serde_yaml::from_reader(File::open(path).unwrap())
    }
}
