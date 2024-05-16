use std::path::PathBuf;

use eyre::Result;

pub fn in_dir_with_ext(
    dir: impl Into<PathBuf>,
    ext: &'static str,
) -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
    Ok(std::fs::read_dir(dir.into())?
        .into_iter()
        .map(|f| f.unwrap().path())
        .filter(move |path| path.file_name().unwrap().to_str().unwrap().ends_with(ext)))
}
