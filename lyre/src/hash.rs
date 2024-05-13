use std::{
    fs,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use eyre::{Ok, Result};
use sha2::{Digest, Sha256};

fn generate_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut hasher = Sha256::new();

    hasher.update(fs::read(path)?);

    Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
}

pub fn process() -> Result<()> {
    println!("Processing Version Hashes...");

    let paths = vec![
        "generated/static/favicon.ico",
        "generated/static/main.js",
        "generated/static/styles.css",
    ];

    for path in paths {
        let path: PathBuf = path.into();

        fs::write(
            Path::new("generated/hashes/")
                .join(path.file_name().unwrap())
                .with_extension(match path.extension() {
                    Some(e) => {
                        let mut e = e.to_os_string();
                        e.push(".hash");
                        e
                    }
                    None => "hash".into(),
                }),
            generate_hash(path)?,
        )?;
    }

    Ok(())
}
