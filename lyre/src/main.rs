use eyre::{Ok, Result};

mod hash;
mod posts;
mod web;

fn main() -> Result<()> {
    color_eyre::install()?;
    web::directories()?;
    web::javascript()?;
    web::favicon()?;
    web::scss()?;
    posts::process()?;
    hash::process()?;
    println!("Done!");
    Ok(())
}
