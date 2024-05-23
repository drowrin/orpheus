use std::time::SystemTime;

use color_eyre::owo_colors::OwoColorize;
use colored::*;
use eyre::{Ok, Result};
use melody::{finalize, Melody};

pub mod pages;
pub mod posts;
pub mod web;

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let started = SystemTime::now();

    let mut state = melody::prepare()?;

    <web::Parcel as Melody>::conduct(&mut state)?;
    <web::Favicon as Melody>::conduct(&mut state)?;
    <web::SCSS as Melody>::conduct(&mut state)?;
    <web::Images as Melody>::conduct(&mut state)?;
    <posts::Posts as Melody>::conduct(&mut state)?;
    <pages::Pages as Melody>::conduct(&mut state)?;

    finalize(state)?;

    println!("{} in {:?}", "Done".green(), started.elapsed()?.yellow());
    Ok(())
}
