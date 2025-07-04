use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    result::Result as stdResult,
};

use chrono::Utc;
use colored::*;
use eyre::{eyre, Context, Result};
use glob::glob;
use inquire::{
    autocompletion::Replacement,
    validator::{ErrorMessage, Validation},
    Autocomplete, CustomUserError, MultiSelect, Select, Text,
};

use clap::{Parser, Subcommand};
use simple_search::{
    levenshtein::base::weighted_levenshtein_similarity, search_engine::SearchEngine,
};
use slug::slugify;
use tantivy::{doc, IndexWriter};
use verse::{Frontmatter, SearchMeta};

#[derive(Parser)]
#[command(version, about, infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Default)]
enum Commands {
    /// Default command. Builds all Orpheus content
    #[default]
    Build,

    /// Generate new empty content files from templates
    Gen,
}

#[derive(Clone)]
struct AutoCompleter<FS, FC>
where
    FS: Clone + Fn(&str) -> stdResult<Vec<String>, CustomUserError>,
    FC: Clone + Fn(&str, Option<String>) -> stdResult<Replacement, CustomUserError>,
{
    suggestions: FS,
    completion: FC,
}
impl<FS, FC> Autocomplete for AutoCompleter<FS, FC>
where
    FS: Clone + Fn(&str) -> stdResult<Vec<String>, CustomUserError>,
    FC: Clone + Fn(&str, Option<String>) -> stdResult<Replacement, CustomUserError>,
{
    fn get_suggestions(&mut self, input: &str) -> stdResult<Vec<String>, CustomUserError> {
        (self.suggestions)(input)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> stdResult<Replacement, CustomUserError> {
        (self.completion)(input, highlighted_suggestion)
    }
}

fn ask_frontmatter(all_frontmatter: Vec<Frontmatter>) -> Result<Frontmatter> {
    let series_search_engine = SearchEngine::new()
        .with_values(
            all_frontmatter
                .iter()
                .flat_map(|f| f.series.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect(),
        )
        .with(|a, b| weighted_levenshtein_similarity(b, a));
    let series: String = Text::new("Series:")
        .with_autocomplete(AutoCompleter {
            suggestions: move |input| {
                Ok(series_search_engine
                    .search(input)
                    .into_iter()
                    .map(|s| s.to_owned())
                    .rev()
                    .collect())
            },
            completion: |_, highlighted_suggestion| Ok(highlighted_suggestion),
        })
        .with_help_message("Leave blank for none")
        .prompt()?;

    let titles_search_engine = SearchEngine::new()
        .with_values(
            all_frontmatter
                .iter()
                .filter(|f| f.series == Some(series.clone()))
                .map(|f| f.title.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect(),
        )
        .with(|a, b| weighted_levenshtein_similarity(b, a));
    let title: String = Text::new("Title:")
        .with_autocomplete(AutoCompleter {
            suggestions: move |input| {
                Ok(titles_search_engine
                    .search(input)
                    .into_iter()
                    .map(|s| s.to_owned())
                    .rev()
                    .collect())
            },
            completion: |_, highlighted_suggestion| Ok(highlighted_suggestion),
        })
        .with_validator(|input: &str| {
            Ok({
                if input.is_empty() {
                    Validation::Invalid(ErrorMessage::Custom("Please enter a title!".to_string()))
                } else {
                    Validation::Valid
                }
            })
        })
        .prompt()?;

    let all_tags = all_frontmatter
        .iter()
        .flat_map(|f| f.tags.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let tags: Vec<String> = MultiSelect::new("Tags:", all_tags).prompt()?;

    let series = if !series.is_empty() {
        Some(series)
    } else {
        None
    };

    Ok(Frontmatter {
        title,
        series: series.clone(),
        tags,
        published: Utc::now().format("%F").to_string(),
        ..Default::default()
    })
}

fn gen_post(frontmatter: Frontmatter) -> Result<()> {
    let mut path = Path::new("content").join("posts").to_path_buf();

    if let Some(series) = frontmatter.series.clone() {
        path = path.join(slugify(series));
    }

    path = path
        .join(slugify(frontmatter.title.clone()))
        .with_extension("md");

    let templates: Vec<PathBuf> = glob("content/templates/**/*.md")?.flatten().collect();
    let template_map: HashMap<String, PathBuf> = templates
        .clone()
        .into_iter()
        .flat_map(|t| Some((t.with_extension("").file_name()?.to_str()?.to_string(), t)))
        .collect();
    let template_keys: Vec<&String> = template_map.keys().collect();

    let selected_template = Select::new("Template:", template_keys).prompt()?;

    fs::create_dir_all(path.with_file_name(""))?;
    fs::write(
        &path,
        format!(
            "---\n{}---\n\n{}",
            serde_yaml::to_string(&frontmatter)?.replace(" null", ""),
            fs::read_to_string(template_map.get(selected_template).unwrap())?,
        ),
    )?;

    println!("Generated at: {}", path.to_str().unwrap().blue());

    Ok(())
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

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let all_frontmatter: Vec<Frontmatter> = glob("content/posts/**/*.md")?
        .flatten()
        .flat_map(|p| {
            serde_yaml::from_str::<Frontmatter>(
                fs::read_to_string(p)
                    .ok()?
                    .split("---")
                    .collect::<Vec<_>>()
                    .get(1)?,
            )
            .ok()
        })
        .collect();

    match args.command {
        Commands::Build => {
            let search_meta = SearchMeta::open()?;
            let mut index_writer: IndexWriter = search_meta.index.writer(50_000_000)?;
            let paths = glob("content/posts/**/*.md")?;
            for path in paths {
                let path = path.unwrap();
                if matches!(path.extension(), Some(ext) if ext == "md") {
                    let name = path.file_name().unwrap();

                    let fm = parse_frontmatter(&path)?;

                    let plain_path = Path::new("generated")
                        .join("posts")
                        .join(name)
                        .with_extension("txt");
                    let raw_text = fs::read_to_string(&plain_path)?;

                    index_writer.add_document(doc!(
                        search_meta.fields.title => fm.title,
                        search_meta.fields.tagline => fm.tagline.unwrap_or_default(),
                        search_meta.fields.body => raw_text,
                        search_meta.fields.slug => name.to_str().unwrap().strip_suffix(".md").unwrap().to_string(),
                    ))?;
                }
            }

            index_writer.commit()?;
        }
        Commands::Gen => gen_post(ask_frontmatter(all_frontmatter)?)?,
    }

    Ok(())
}
