use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};
use tantivy::{
    directory::MmapDirectory,
    query::QueryParser,
    schema::{Field, Schema, FAST, STORED, TEXT},
    Index, IndexReader, TantivyError,
};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct ToCDepth(pub u32);
impl Default for ToCDepth {
    fn default() -> Self {
        Self(3)
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub brief: Option<String>,
    pub tagline: Option<String>,
    pub series: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub published: String,
    pub updated: Option<String>,
    #[serde(default)]
    pub toc_depth: ToCDepth,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Series {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostMetaData {
    pub title: String,
    pub slug: String,
    pub brief: String,
    pub tagline: Option<String>,
    pub series: Option<Series>,
    pub tags: Vec<String>,
    pub word_count: usize,
    pub reading_time: usize,
    pub published: String,
    pub updated: String,
    pub revisions: String,
    pub toc_depth: u32,
}

impl PostMetaData {
    pub fn open<P>(path: P) -> Result<Self, impl Error>
    where
        P: AsRef<Path>,
    {
        serde_yaml::from_reader(File::open(path).unwrap())
    }
}

pub struct SearchFields {
    pub title: Field,
    pub tagline: Field,
    pub body: Field,
    pub slug: Field,
}

pub struct SearchMeta {
    pub schema: Schema,
    pub index: Index,
    pub reader: IndexReader,
    pub fields: SearchFields,
    pub parser: QueryParser,
}

impl SearchMeta {
    pub fn open() -> Result<Self, TantivyError> {
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT | FAST);
        let tagline = schema_builder.add_text_field("tagline", TEXT | FAST);
        let body = schema_builder.add_text_field("body", TEXT | FAST);
        let slug = schema_builder.add_text_field("slug", STORED | FAST);
        let schema = schema_builder.build();
        fs::create_dir_all("generated/posts/index")?;
        let index = Index::open_or_create(
            MmapDirectory::open("generated/posts/index")?,
            schema.clone(),
        )?;

        let fields = SearchFields {
            title,
            tagline,
            body,
            slug,
        };

        let reader = index.reader()?;

        let mut parser = QueryParser::for_index(&index, vec![title, tagline, body]);
        parser.set_field_boost(title, 5.0);
        parser.set_field_boost(tagline, 2.0);
        parser.set_field_fuzzy(title, false, 2, true);
        parser.set_conjunction_by_default();

        Ok(Self {
            schema,
            index,
            reader,
            fields,
            parser,
        })
    }
}
