use std::{fs, path::Path, time::SystemTime};

use eyre::{eyre, Ok, Result, WrapErr};

pub fn directories() -> Result<()> {
    fs::create_dir_all("generated/posts/")?;
    fs::create_dir_all("generated/static/")?;
    fs::create_dir_all("generated/hashes/")?;

    Ok(())
}

pub fn prism_components() -> Result<()> {
    fs::create_dir_all("generated/static/components")?;

    for path in fs::read_dir("./node_modules/prismjs/components")? {
        let path = path?.path();
        let name = path.file_name().unwrap();
        if name.to_str().unwrap().ends_with(".min.js") {
            fs::copy(&path, Path::new("generated/static/components/").join(name))?;
        }
    }

    Ok(())
}

pub fn javascript() -> Result<()> {
    println!("Running parcel...");
    let start = SystemTime::now();

    let output = std::process::Command::new("npm.cmd")
        .args([
            "exec",
            "--",
            "parcel",
            "build",
            "--dist-dir",
            "./generated/static",
            "./web/main.js",
        ])
        .output()?;

    if !output.status.success() {
        return Err(eyre!("{}", String::from_utf8(output.stderr)?));
    }

    println!("Running parcel took {:?}", start.elapsed()?);

    Ok(())
}

pub fn favicon() -> Result<()> {
    println!("Copying favicon...");
    let start = SystemTime::now();

    std::fs::copy("content/favicon.ico", "generated/static/favicon.ico")
        .wrap_err("content/favicon.ico is missing")?;

    println!("Copying favicon took {:?}", start.elapsed()?);

    Ok(())
}

pub fn scss() -> Result<()> {
    println!("Processing SCSS...");
    let start = SystemTime::now();

    let css = grass::from_path(
        "web/styles.scss",
        &grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .load_path("node_modules/@picocss/pico/scss/")
            .load_path("node_modules/@catppuccin/palette/scss"),
    )?;

    fs::write("generated/static/styles.css", css)?;

    println!("Processing SCSS took {:?}", start.elapsed()?);

    Ok(())
}
