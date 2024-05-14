use std::time::SystemTime;

use colored::*;
use eyre::{Ok, Result};

mod hash;
mod posts;
mod web;

fn main() -> Result<()> {
    color_eyre::install()?;

    let started = SystemTime::now();

    web::directories()?;
    web::prism_components()?;
    web::javascript()?;
    web::favicon()?;
    web::scss()?;
    posts::process()?;
    hash::process()?;

    println!("{} in {:?}", "Done!".green(), started.elapsed()?);
    Ok(())
}
