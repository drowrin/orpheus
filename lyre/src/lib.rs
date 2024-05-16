use std::time::SystemTime;

use color_eyre::owo_colors::OwoColorize;
use colored::*;
use eyre::{Ok, Result};
use melody::Melody;

pub mod posts;
pub mod web;

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let started = SystemTime::now();

    melody::prepare()?;

    <web::Parcel as Melody>::conduct()?;
    <web::Favicon as Melody>::conduct()?;
    <web::SCSS as Melody>::conduct()?;
    <posts::Posts as Melody>::conduct()?;

    println!("{} in {:?}", "Done".green(), started.elapsed()?.yellow());
    Ok(())
}
