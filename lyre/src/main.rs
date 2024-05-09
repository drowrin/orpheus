use eyre::{eyre, Result};
use lyre::{MetaData, Series};
use serde::Deserialize;
use slug::slugify;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct Frontmatter {
    pub title: String,
    pub tagline: Option<String>,
    pub series: Option<String>,
    pub tags: Vec<String>,
}

fn main() -> Result<()> {
    for path in fs::read_dir("./content/posts")? {
        let path = path.unwrap().path();
        if matches!(path.extension(), Some(ext) if ext == "md") {
            let name = path.file_name().unwrap();
            let file = fs::read_to_string(&path)?;
            let fm: Frontmatter = serde_yaml::from_str(file.split("---").skip(1).next().ok_or(
                eyre!("Could not locate frontmatter in {}", path.to_str().unwrap()),
            )?)?;

            let metadata = MetaData {
                title: fm.title,
                slug: name.to_str().unwrap().strip_suffix(".md").unwrap().into(),
                tagline: fm.tagline,
                series: fm.series.map(|s| Series {
                    name: s.clone(),
                    slug: slugify(s),
                }),
                tags: fm.tags.iter().map(|t| slugify(t)).collect(),
            };

            fs::write(
                Path::new("./generated/posts")
                    .join(name)
                    .with_extension("yml"),
                serde_yaml::to_string(&metadata)?,
            )?;
        }
    }

    Ok(())
}
