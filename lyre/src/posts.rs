use eyre::{eyre, Ok, Result, WrapErr};

use lyre::{Frontmatter, MetaData, Series};
use pandoc::MarkdownExtension;
use slug::slugify;
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn render_html(from: &PathBuf) -> Result<PathBuf> {
    let target = Path::new("./generated/posts/")
        .join(from.file_name().unwrap())
        .with_extension("html");

    let mut doc = pandoc::new();
    doc.add_input(from);
    doc.set_input_format(
        pandoc::InputFormat::CommonmarkX,
        vec![
            MarkdownExtension::Attributes,
            MarkdownExtension::ImplicitFigures,
            MarkdownExtension::AutolinkBareUris,
        ],
    );
    doc.add_option(pandoc::PandocOption::LuaFilter("pandoc/filters.lua".into()));
    doc.add_option(pandoc::PandocOption::LuaFilter(
        "pandoc/standard-code.lua".into(),
    ));
    doc.add_option(pandoc::PandocOption::NoHighlight);
    doc.set_output(pandoc::OutputKind::File(target.clone()));
    doc.set_output_format(
        pandoc::OutputFormat::Html5,
        vec![
            MarkdownExtension::TaskLists,
            MarkdownExtension::AsciiIdentifiers,
        ],
    );
    doc.execute()?;

    Ok(target)
}

pub fn render_plain(from: &PathBuf) -> Result<PathBuf> {
    let target = Path::new("./generated/posts/")
        .join(from.file_name().unwrap())
        .with_extension("txt");

    let mut doc = pandoc::new();
    doc.add_input(from);
    doc.set_input_format(
        pandoc::InputFormat::Commonmark,
        vec![
            MarkdownExtension::Attributes,
            MarkdownExtension::YamlMetadataBlock,
        ],
    );
    doc.add_option(pandoc::PandocOption::NoWrap);
    doc.set_output(pandoc::OutputKind::File(target.clone()));
    doc.set_output_format(pandoc::OutputFormat::Plain, vec![]);
    doc.execute()?;

    Ok(target)
}

pub fn parse_frontmatter(path: &PathBuf) -> Result<Frontmatter> {
    let md = fs::read_to_string(&path)?;
    let mut sections = md.split("---").skip(1);

    let frontmatter = serde_yaml::from_str(sections.next().ok_or(eyre!(
        "Could not locate frontmatter in {}",
        path.to_str().unwrap()
    ))?)
    .wrap_err(format!("In file: {}", path.to_str().unwrap()))?;

    // ensure we've at least got some content
    sections.next().ok_or(eyre!(
        "Could not locate content in {}",
        path.to_str().unwrap()
    ))?;

    Ok(frontmatter)
}

pub fn process_plain(path: &PathBuf) -> Result<(usize, String)> {
    let plain_path = render_plain(&path)?;

    let plain = fs::read_to_string(&plain_path)
        .wrap_err(format!("File: {}", plain_path.to_str().unwrap()))?;
    let first_p = plain
        .lines()
        .filter(|l| !l.starts_with(['#']) && l.len() > 0)
        .next()
        .ok_or(eyre!("Empty content in {}", plain_path.to_str().unwrap()))?;

    Ok((plain.split_whitespace().count(), first_p.to_string()))
}

pub fn process() -> Result<()> {
    println!("Rendering posts...");
    let start = SystemTime::now();

    for path in fs::read_dir("./content/posts")? {
        let path = path.unwrap().path();

        if matches!(path.extension(), Some(ext) if ext == "md") {
            render_html(&path)?;

            let name = path.file_name().unwrap();

            let fm = parse_frontmatter(&path)?;

            let (word_count, first_p) = process_plain(&path)?;

            let metadata = MetaData {
                title: fm.title,
                slug: name.to_str().unwrap().strip_suffix(".md").unwrap().into(),
                brief: fm.brief.unwrap_or(first_p),
                tagline: fm.tagline,
                series: fm.series.map(|s| Series {
                    name: s.clone(),
                    slug: slugify(s),
                }),
                tags: fm.tags.iter().map(|t| slugify(t)).collect(),
                word_count,
                reading_time: word_count / 240,
                published: fm.published,
                updated: fm.updated,
            };

            fs::write(
                Path::new("./generated/posts")
                    .join(name)
                    .with_extension("yml"),
                serde_yaml::to_string(&metadata)?,
            )?;
        }
    }

    println!("Rendering posts took {:?}", start.elapsed()?);

    Ok(())
}
