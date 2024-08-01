use std::{collections::HashSet, fs, path::Path, result::Result as stdResult, time::SystemTime};

use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use colored::*;
use eyre::Result;
use glob::glob;
use inquire::{autocompletion::Replacement, Autocomplete, CustomUserError, Text};
use melody::{finalize, Melody};

use clap::{Parser, Subcommand};
use simple_search::{
    levenshtein::base::weighted_levenshtein_similarity, search_engine::SearchEngine,
};
use slug::slugify;
use verse::Frontmatter;

pub mod pages;
pub mod posts;
pub mod web;

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
    Gen {
        #[command(subcommand)]
        template: Templates,
    },
}

#[derive(Subcommand)]
enum Templates {
    /// Generates an empty post
    Post {
        /// Title of the post. Also used to generate the file name
        #[arg(num_args(..), trailing_var_arg(true),)]
        title: Vec<String>,
    },
    /// Generates an empty review post
    Review {
        /// Title of the post. Also used to generate the file name
        #[arg(num_args(..), trailing_var_arg(true),)]
        title: Vec<String>,
    },
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

fn ask_frontmatter(title: String, all_frontmatter: Vec<Frontmatter>) -> Result<Frontmatter> {
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
    let tags_search_engine = SearchEngine::new()
        .with_values(
            all_frontmatter
                .iter()
                .flat_map(|f| f.tags.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect(),
        )
        .with(|a, b| weighted_levenshtein_similarity(a, b));

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

    let tags: Vec<String> = Text::new("Tags:")
        .with_autocomplete(AutoCompleter {
            suggestions: move |input| {
                Ok(tags_search_engine
                    .search(input)
                    .into_iter()
                    .map(|s| s.to_owned())
                    .rev()
                    .collect())
            },
            completion: |input, highlighted_suggestion| {
                Ok(highlighted_suggestion.map(|suggestion| {
                    input
                        .split(&[',', ' ', '\t'])
                        .filter(|s| !s.is_empty())
                        .chain([suggestion.as_str()])
                        .collect::<Vec<&str>>()
                        .join(" ")
                }))
            },
        })
        .with_help_message("Leave blank for none")
        .prompt()?
        .split(&[',', ' ', '\t'])
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

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

fn gen_post(frontmatter: Frontmatter, template: &str) -> Result<()> {
    let mut path = Path::new("content").join("posts").to_path_buf();

    if let Some(series) = frontmatter.series.clone() {
        path = path.join(slugify(series));
    }

    path = path
        .join(slugify(frontmatter.title.clone()))
        .with_extension("md");

    fs::create_dir_all(path.with_file_name(""))?;
    fs::write(
        &path,
        format!(
            "---\n{}---\n\n{}",
            serde_yaml::to_string(&frontmatter)?.replace(" null", ""),
            template,
        ),
    )?;

    println!("Generated at: {}", path.to_str().unwrap().blue());

    Ok(())
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
            let started = SystemTime::now();

            let mut state = melody::prepare()?;

            <web::Javascript as Melody>::conduct(&mut state)?;
            <web::Favicon as Melody>::conduct(&mut state)?;
            <web::SCSS as Melody>::conduct(&mut state)?;
            <web::Images as Melody>::conduct(&mut state)?;
            <posts::Posts as Melody>::conduct(&mut state)?;
            <pages::Pages as Melody>::conduct(&mut state)?;

            finalize(state)?;

            println!(
                "{} in {:?}",
                "Lyre Completed".green(),
                started.elapsed()?.yellow()
            );
        }
        Commands::Gen { template } => match template {
            Templates::Post { title } => gen_post(
                ask_frontmatter(title.join(" "), all_frontmatter)?,
                include_str!("../../templates/post.md"),
            )?,
            Templates::Review { title } => gen_post(
                ask_frontmatter(title.join(" "), all_frontmatter)?,
                include_str!("../../templates/review.md"),
            )?,
        },
    }

    Ok(())
}
