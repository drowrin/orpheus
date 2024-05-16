use std::{
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

pub const HASH_PATHS: &[&str] = &[
    "web/styles.scss",
    "web/entrypoint.js",
    "generated/static/favicon.ico",
    "generated/static/main.js",
    "generated/static/styles.css",
];

pub fn generate_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut hasher = Sha256::new();

    hasher.update(fs::read(path)?);

    Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
}

pub fn hashed_path(path: &PathBuf) -> PathBuf {
    Path::new("generated/hashes/")
        .join(path.file_name().unwrap())
        .with_extension(match path.extension() {
            Some(e) => {
                let mut e = e.to_os_string();
                e.push(".hash");
                e
            }
            None => "hash".into(),
        })
}

pub fn check_hashes() -> Result<bool, std::io::Error> {
    for path in HASH_PATHS {
        let path: PathBuf = path.into();
        let hashed_path = hashed_path(&path);
        let hash = generate_hash(path)?;
        let saved_hash = fs::read_to_string(hashed_path)?;

        if hash != saved_hash {
            return Ok(false);
        }
    }

    Ok(true)
}
