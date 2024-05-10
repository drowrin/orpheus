use eyre::{eyre, Result, WrapErr};
use lyre::{MetaData, Series};
use serde::Deserialize;
use slug::slugify;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct Frontmatter {
    pub title: String,
    pub brief: Option<String>,
    pub tagline: Option<String>,
    pub series: Option<String>,
    pub tags: Vec<String>,
}

fn main() -> Result<()> {
    for path in fs::read_dir("./content/posts")? {
        let path = path.unwrap().path();
        if matches!(path.extension(), Some(ext) if ext == "md") {
            let name = path.file_name().unwrap();
            let md = fs::read_to_string(&path)?;
            let mut sections = md.split("---").skip(1);
            let fm: Frontmatter = serde_yaml::from_str(sections.next().ok_or(eyre!(
                "Could not locate frontmatter in {}",
                path.to_str().unwrap()
            ))?)
            .wrap_err(format!("In file: {}", path.to_str().unwrap()))?;
            sections.next().ok_or(eyre!(
                "Could not locate content in {}",
                path.to_str().unwrap()
            ))?;

            let mut plain_path = Path::new("./generated/posts/*").with_file_name(name);
            plain_path.set_extension("txt");
            let plain = fs::read_to_string(&plain_path)
                .wrap_err(format!("File: {}", plain_path.to_str().unwrap()))?;
            let first_p = plain
                .lines()
                .filter(|l| !l.starts_with(['#']) && l.len() > 0)
                .next()
                .ok_or(eyre!("Empty content in {}", path.to_str().unwrap()))?;

            let metadata = MetaData {
                title: fm.title,
                slug: name.to_str().unwrap().strip_suffix(".md").unwrap().into(),
                brief: fm.brief.unwrap_or(first_p.to_string()),
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
