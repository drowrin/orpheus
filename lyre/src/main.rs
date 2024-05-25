use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
    time::SystemTime,
};

use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use colored::*;
use eyre::{Ok, Result};
use melody::{finalize, Melody};

use clap::{Parser, Subcommand};
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
}

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Commands::Build => {
            let started = SystemTime::now();

            let mut state = melody::prepare()?;

            <web::Parcel as Melody>::conduct(&mut state)?;
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
            Templates::Post { title } => {
                print!("Series (leave blank for none): ");
                stdout().flush()?;
                let mut series = String::new();
                stdin().read_line(&mut series)?;
                series = series.trim().to_string();

                let series = if !series.is_empty() {
                    Some(series)
                } else {
                    None
                };

                let title = title.join(" ");
                let frontmatter = Frontmatter {
                    title: title.clone(),
                    series: series.clone(),
                    published: Utc::now().format("%F").to_string(),
                    ..Default::default()
                };

                let mut path = Path::new("content").join("posts").to_path_buf();

                if let Some(series) = series {
                    path = path.join(slugify(series));
                }

                path = path.join(slugify(title)).with_extension("md");

                fs::create_dir_all(path.with_file_name(""))?;
                fs::write(
                    &path,
                    format!(
                        "---\n{}---\n\n# Content Here\n",
                        serde_yaml::to_string(&frontmatter)?.replace(" null", "")
                    ),
                )?;

                println!("Generated at: {}", path.to_str().unwrap().blue());
            }
        },
    }

    Ok(())
}
