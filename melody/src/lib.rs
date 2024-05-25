use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use colored::*;
use eyre::{Context, Result};
use sha2::{Digest, Sha256};

pub type ETags = HashMap<String, String>;

pub fn prepare() -> Result<ETags> {
    fs::create_dir_all("generated/repertoire/")?;
    Ok(ETags::new())
}

pub fn finalize(etags: ETags) -> Result<()> {
    fs::write(
        "generated/etags",
        etags
            .into_iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    Ok(())
}

pub trait Melody {
    fn name() -> &'static str;
    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn perform(parts: impl Iterator<Item = PathBuf>) -> Result<()>;

    fn generate_hash(path: &PathBuf) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(fs::read(path).context(format!("Path: {:?}", path))?);

        Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
    }

    fn input_hash_path(path: &Path) -> PathBuf {
        Path::new("./generated/repertoire/")
            .join(Self::name())
            .join(path.file_name().unwrap())
            .with_extension("hash")
    }

    fn ready(path: &PathBuf) -> Result<bool> {
        let hash = match fs::read_to_string(Self::input_hash_path(path)) {
            Ok(sheet) => sheet,
            Err(_) => return Ok(false),
        };

        if Self::generate_hash(path)? != hash {
            return Ok(false);
        }

        Ok(true)
    }

    fn make_etags(etags: &mut ETags) -> Result<()> {
        let output_path_set = Self::rendition()?
            .into_iter()
            .map(|p| p.into())
            .collect::<HashSet<_>>();
        for rendition_path in output_path_set {
            let hash_path = rendition_path
                .with_extension("")
                .iter()
                .filter(|p| {
                    let p = p.to_str().unwrap();
                    p != "." && p != "generated" && p != "pages" && p != "static"
                })
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>()
                .join("/");
            etags.insert(
                format!("/{hash_path}"),
                Self::generate_hash(&rendition_path)?,
            );
        }

        Ok(())
    }

    fn conduct(etags: &mut ETags) -> Result<()> {
        let needs_rebuild: Vec<PathBuf> = Self::source()?
            .into_iter()
            .map(|p| p.into())
            .filter(|p| {
                if let Ok(v) = Self::ready(p) {
                    return !v;
                }
                true
            })
            .collect();

        if needs_rebuild.is_empty() {
            return Self::make_etags(etags);
        }

        print!("Rebuilding {}", Self::name());
        std::io::stdout().flush().unwrap();

        let path_set = Self::rendition()?
            .into_iter()
            .map(|p| p.into().with_file_name(""))
            .collect::<HashSet<_>>();
        for path in path_set {
            fs::create_dir_all(path)?;
        }

        let started = SystemTime::now();

        Self::perform(needs_rebuild.clone().into_iter())?;

        println!(
            " took {}",
            format!("{:?}", started.elapsed().unwrap()).yellow()
        );
        for path in needs_rebuild {
            fs::create_dir_all(Self::input_hash_path(&path).with_file_name(""))?;
            fs::write(Self::input_hash_path(&path), Self::generate_hash(&path)?)?;
        }

        Self::make_etags(etags)
    }
}
