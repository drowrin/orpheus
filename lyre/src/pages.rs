use std::{
    fs,
    path::{Path, PathBuf},
};

use melody::{utils::in_dir_with_ext, Melody};
use pandoc::MarkdownExtension;

use crate::posts::code_highlighting;

pub struct Pages;
impl Melody for Pages {
    fn name() -> &'static str {
        "Pages"
    }

    fn source() -> eyre::Result<impl IntoIterator<Item = impl Into<std::path::PathBuf>>> {
        in_dir_with_ext("./content/pages", ".md")
    }

    fn rendition() -> eyre::Result<impl IntoIterator<Item = impl Into<std::path::PathBuf>>> {
        Ok(Self::source()?.into_iter().map(|p| {
            let p: PathBuf = p.into();
            Path::new("./generated/pages/")
                .join(p.file_name().unwrap())
                .with_extension("html")
        }))
    }

    fn perform() -> eyre::Result<()> {
        for path in fs::read_dir("./content/pages")? {
            let path = path.unwrap().path();

            if matches!(path.extension(), Some(ext) if ext == "md") {
                let target = Path::new("./generated/pages/")
                    .join(path.file_name().unwrap())
                    .with_extension("html");

                let mut doc = pandoc::new();
                doc.add_input(&path);
                doc.set_input_format(
                    pandoc::InputFormat::Markdown,
                    vec![
                        MarkdownExtension::LinkAttributes,
                        MarkdownExtension::HeaderAttributes,
                        MarkdownExtension::FencedCodeAttributes,
                        MarkdownExtension::InlineCodeAttributes,
                        MarkdownExtension::FencedDivs,
                        MarkdownExtension::ImplicitFigures,
                        MarkdownExtension::AutolinkBareUris,
                        MarkdownExtension::MarkdownInHtmlBlocks,
                    ],
                );
                doc.add_option(pandoc::PandocOption::LuaFilter(
                    "pandoc/image-rebase.lua".into(),
                ));
                doc.add_option(pandoc::PandocOption::LuaFilter(
                    "pandoc/link-preload.lua".into(),
                ));
                doc.add_filter(code_highlighting);
                doc.add_option(pandoc::PandocOption::NoHighlight);
                doc.set_output(pandoc::OutputKind::File(target));
                doc.set_output_format(
                    pandoc::OutputFormat::Html5,
                    vec![
                        MarkdownExtension::TaskLists,
                        MarkdownExtension::AsciiIdentifiers,
                    ],
                );
                doc.execute()?;
            }
        }

        Ok(())
    }
}
