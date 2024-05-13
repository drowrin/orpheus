use std::fs;

use eyre::{Ok, Result, WrapErr};

pub fn directories() -> Result<()> {
    std::fs::create_dir_all("generated/posts/")?;
    std::fs::create_dir_all("generated/static/")?;
    std::fs::create_dir_all("generated/hashes/")?;

    Ok(())
}

pub fn javascript() -> Result<()> {
    println!("Running Parcel...");
    std::process::Command::new("npm.cmd")
        .args([
            "exec",
            "--",
            "parcel",
            "build",
            "--dist-dir",
            "./generated/static",
            "./web/entrypoint.js",
        ])
        .output()?;

    Ok(())
}

pub fn favicon() -> Result<()> {
    println!("Copying Favicon...");
    std::fs::copy("content/favicon.ico", "generated/static/favicon.ico")
        .wrap_err("content/favicon.ico is missing")?;

    Ok(())
}

pub fn scss() -> Result<()> {
    println!("Processing SCSS...");
    let css = grass::from_path(
        "web/styles.scss",
        &grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .load_path("node_modules/@picocss/pico/scss/"),
    )?;

    fs::write("generated/static/styles.css", css)?;

    Ok(())
}
