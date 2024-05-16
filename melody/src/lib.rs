use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use colored::*;
use eyre::{Context, Result};
use sha2::{Digest, Sha256};

pub mod utils;

pub fn prepare() -> Result<()> {
    fs::create_dir_all("generated/repertoire/")?;
    Ok(())
}

pub trait Melody {
    fn name() -> &'static str;
    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn perform() -> Result<()>;

    fn generate_hash(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Result<String> {
        let mut hasher = Sha256::new();

        for path in paths {
            let path = path.into();
            hasher.update(fs::read(&path).context(format!("Path: {:?}", path))?);
        }

        Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
    }

    fn source_hash() -> Result<String> {
        Self::generate_hash(Self::source()?)
    }

    fn path() -> PathBuf {
        Path::new("./generated/repertoire/")
            .join(Self::name())
            .with_extension("hash")
    }

    fn read() -> Result<String> {
        Ok(fs::read_to_string(Self::path())?)
    }

    fn write(hash: String) -> Result<()> {
        fs::write(Self::path(), hash)?;

        Ok(())
    }

    fn ready() -> Result<bool> {
        let hash = match Self::read() {
            Ok(sheet) => sheet,
            Err(_) => return Ok(false),
        };

        if Self::source_hash()? != hash {
            return Ok(false);
        }

        Ok(true)
    }

    fn conduct() -> Result<()> {
        print!("{}", Self::name());

        let hash = match Self::read() {
            Ok(sheet) => sheet,
            Err(_) => String::default(),
        };

        let source = Self::source_hash()?;
        if source == hash {
            println!("{}", " cached".green());
            return Ok(());
        }

        let path_set = Self::rendition()?
            .into_iter()
            .map(|p| p.into().with_file_name(""))
            .collect::<HashSet<_>>();
        for path in path_set {
            fs::create_dir_all(path)?;
        }

        let started = SystemTime::now();

        Self::perform()?;

        println!(
            " took {}",
            format!("{:?}", started.elapsed().unwrap()).yellow()
        );

        Self::write(source)
    }
}
