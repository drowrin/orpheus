use eyre::{Ok, Result};

mod hash;
mod posts;
mod web;

fn main() -> Result<()> {
    web::directories()?;
    web::javascript()?;
    web::favicon()?;
    web::scss()?;
    posts::process()?;
    hash::process()?;
    println!("Done!");
    Ok(())
}
