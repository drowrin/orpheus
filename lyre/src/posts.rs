use eyre::{eyre, Ok, Result, WrapErr};
use glob::glob;
use melody::Melody;
use pandoc::MarkdownExtension;
use pandoc_ast::Block;
use slug::slugify;
use std::{
    fs,
    path::{Path, PathBuf},
};
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use verse::{Frontmatter, PostMetaData, Series};

pub fn code_highlighting(json: String) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();

    pandoc_ast::filter(json, |mut pandoc| {
        for block in &mut pandoc.blocks {
            *block = match block {
                Block::CodeBlock((ref id, ref classes, ref attrs), content) => {
                    if classes.len() > 0 {
                        let language = classes.first().unwrap();
                        let syntax = syntax_set
                            .find_syntax_by_extension(&language)
                            .or(syntax_set.find_syntax_by_name(&language))
                            .expect(format!("Unrecognized language \"{}\"", language).as_str());
                        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                            syntax,
                            &syntax_set,
                            ClassStyle::SpacedPrefixed { prefix: "c-" },
                        );
                        for line in LinesWithEndings::from(content) {
                            html_generator
                                .parse_html_for_line_which_includes_newline(line)
                                .unwrap();
                        }
                        let code = html_generator.finalize();
                        let mut pre_classes = classes.join(" ");
                        if pre_classes.len() > 0 {
                            pre_classes = format!(" class=\"{}\"", pre_classes);
                        }
                        let code_classes = format!("language-{}", language);
                        Block::RawBlock(
                            pandoc_ast::Format("html5".into()),
                            format!(
                                "<pre{}><code class=\"{}\">{}\n</code></pre>",
                                pre_classes, code_classes, code
                            ),
                        )
                    } else {
                        Block::CodeBlock(
                            (id.to_owned(), classes.to_owned(), attrs.to_owned()),
                            content.to_owned(),
                        )
                    }
                }
                _ => block.to_owned(),
            }
        }
        pandoc
    })
}

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
            MarkdownExtension::Subscript,
            MarkdownExtension::Superscript,
        ],
    );
    doc.add_option(pandoc::PandocOption::LuaFilter(
        "pandoc/header-links.lua".into(),
    ));
    doc.add_option(pandoc::PandocOption::LuaFilter(
        "pandoc/image-rebase.lua".into(),
    ));
    doc.add_option(pandoc::PandocOption::LuaFilter(
        "pandoc/link-preload.lua".into(),
    ));
    doc.add_option(pandoc::PandocOption::LuaFilter(
        "pandoc/spoilers.lua".into(),
    ));
    doc.add_filter(code_highlighting);
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

pub struct Posts;
impl Melody for Posts {
    fn name() -> &'static str {
        "Posts"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(glob("content/posts/**/*.md")?.flatten())
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Self::source().map(|source| {
            source.into_iter().flat_map(|path| {
                let path = path.into();
                ["html", "txt", "yml"].into_iter().map(move |ext| {
                    Path::new("./generated/posts/")
                        .join(path.file_name().unwrap())
                        .with_extension(ext)
                })
            })
        })
    }

    fn perform(parts: impl Iterator<Item = PathBuf>) -> Result<()> {
        for path in parts {
            if matches!(path.extension(), Some(ext) if ext == "md") {
                render_html(&path)?;

                let name = path.file_name().unwrap();

                let fm = parse_frontmatter(&path)?;

                let (word_count, first_p) = process_plain(&path)?;

                let metadata = PostMetaData {
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

        Ok(())
    }
}
