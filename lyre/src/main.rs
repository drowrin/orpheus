use std::{
    fs,
    path::{Path, PathBuf},
};

use glob::glob;

use tantivy::{doc, IndexWriter};
use verse::{Frontmatter, SearchMeta};

pub fn parse_frontmatter(path: &PathBuf) -> Frontmatter {
    let md = fs::read_to_string(path).unwrap();
    let mut sections = md.split("---").skip(1);

    let frontmatter = serde_yaml::from_str(sections.next().unwrap());

    // ensure we've at least got some content
    sections.next().unwrap();

    frontmatter.unwrap()
}

pub fn main() {
    let search_meta = SearchMeta::open().unwrap();
    let mut index_writer: IndexWriter = search_meta.index.writer(50_000_000).unwrap();
    let paths = glob("content/posts/**/*.md").unwrap();
    for path in paths {
        let path = path.unwrap();
        if matches!(path.extension(), Some(ext) if ext == "md") {
            let name = path.file_name().unwrap();

            let fm = parse_frontmatter(&path);

            let plain_path = Path::new("generated")
                .join("posts")
                .join(name)
                .with_extension("txt");
            let raw_text = fs::read_to_string(&plain_path).unwrap();

            index_writer.add_document(doc!(
                        search_meta.fields.title => fm.title,
                        search_meta.fields.tagline => fm.tagline.unwrap_or_default(),
                        search_meta.fields.body => raw_text,
                        search_meta.fields.slug => name.to_str().unwrap().strip_suffix(".md").unwrap().to_string(),
                    )).unwrap();
        }
    }

    index_writer.commit().unwrap();
}
