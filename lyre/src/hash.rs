use std::{fs, path::PathBuf};

use eyre::{Ok, Result};
use lyre::{generate_hash, hashed_path, HASH_PATHS};

pub fn process() -> Result<()> {
    println!("Processing Version Hashes...");

    for path in HASH_PATHS {
        let path: PathBuf = path.into();

        fs::write(hashed_path(&path), generate_hash(path)?)?;
    }

    Ok(())
}
