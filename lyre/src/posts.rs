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
use tantivy::{doc, IndexWriter};
use verse::{Frontmatter, PostMetaData, SearchMeta, Series};

pub fn code_highlighting(json: String) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();

    pandoc_ast::filter(json, |mut pandoc| {
        for block in &mut pandoc.blocks {
            *block = match block {
                Block::CodeBlock((ref id, ref classes, ref attrs), content) => {
                    if !classes.is_empty() {
                        let language = classes.first().unwrap();
                        let syntax = syntax_set
                            .find_syntax_by_extension(language)
                            .or(syntax_set.find_syntax_by_name(language))
                            .unwrap_or_else(|| panic!("Unrecognized language \"{}\"", language));
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
                        if !pre_classes.is_empty() {
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
            MarkdownExtension::Footnotes,
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

pub fn render_toc(from: &PathBuf) -> Result<PathBuf> {
    let target = Path::new("./generated/posts/")
        .join(format!(
            "{}-toc",
            from.with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        ))
        .with_extension("html");

    let mut doc = pandoc::new();
    doc.add_input(from);
    doc.set_input_format(pandoc::InputFormat::CommonmarkX, vec![]);
    doc.set_output(pandoc::OutputKind::File(target.clone()));
    doc.add_option(pandoc::PandocOption::Standalone);
    doc.add_option(pandoc::PandocOption::Template(
        Path::new("pandoc/toc-only.html5").to_path_buf(),
    ));
    doc.set_toc();
    doc.set_output_format(pandoc::OutputFormat::Html5, vec![]);
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
    let md = fs::read_to_string(path)?;
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

pub fn process_plain(plain_path: &PathBuf) -> Result<(usize, String)> {
    let plain = fs::read_to_string(plain_path)
        .wrap_err(format!("File: {}", plain_path.to_str().unwrap()))?;
    let first_p = plain
        .lines()
        .find(|l| !l.starts_with(['#']) && !l.is_empty())
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
        let search_meta = SearchMeta::open()?;
        let mut index_writer: IndexWriter = search_meta.index.writer(50_000_000)?;

        for path in parts {
            if matches!(path.extension(), Some(ext) if ext == "md") {
                render_html(&path)?;
                render_toc(&path)?;

                let name = path.file_name().unwrap();

                let fm = parse_frontmatter(&path)?;

                let plain_path = render_plain(&path)?;
                let raw_text = fs::read_to_string(&plain_path)?;
                let (word_count, first_p) = process_plain(&plain_path)?;

                let mut tags: Vec<String> = fm.tags.iter().map(slugify).collect();
                tags.sort();

                let metadata = PostMetaData {
                    title: fm.title,
                    slug: name.to_str().unwrap().strip_suffix(".md").unwrap().into(),
                    brief: fm.brief.unwrap_or(first_p),
                    tagline: fm.tagline,
                    series: fm.series.map(|s| Series {
                        name: s.clone(),
                        slug: slugify(s),
                    }),
                    tags,
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

                index_writer.add_document(doc!(
                    search_meta.fields.title => metadata.title,
                    search_meta.fields.tagline => metadata.tagline.unwrap_or_default(),
                    search_meta.fields.body => raw_text,
                    search_meta.fields.slug => metadata.slug,
                ))?;
            }
        }

        index_writer.commit()?;

        Ok(())
    }
}
