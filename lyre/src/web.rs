use std::{
    fs,
    path::{Path, PathBuf},
};

use eyre::{Ok, Result, WrapErr};
use glob::glob;
use melody::Melody;

pub struct Javascript;
impl Melody for Javascript {
    fn name() -> &'static str {
        "Javascript"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["package.json", "package-lock.json", "web/main.js"])
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok([
            "generated/static/main.js",
            "generated/static/preload.js",
            "generated/static/head-support.js",
            "generated/static/htmx.js",
        ])
    }

    fn perform(_: impl Iterator<Item = PathBuf>) -> Result<()> {
        fs::copy("web/main.js", "generated/static/main.js")?;
        fs::copy(
            "node_modules/htmx.org/dist/htmx.js",
            "generated/static/htmx.js",
        )?;
        fs::copy(
            "node_modules/htmx-ext-preload/preload.js",
            "generated/static/preload.js",
        )?;
        fs::copy(
            "node_modules/htmx-ext-head-support/head-support.js",
            "generated/static/head-support.js",
        )?;
        Ok(())
    }
}

pub struct Favicon;
impl Melody for Favicon {
    fn name() -> &'static str {
        "Favicon"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["content/favicon.svg"])
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["generated/static/favicon.svg"])
    }

    fn perform(_: impl Iterator<Item = PathBuf>) -> Result<()> {
        std::fs::copy("content/favicon.svg", "generated/static/favicon.svg")
            .wrap_err("content/favicon is missing")?;

        Ok(())
    }
}

pub struct SCSS;
impl Melody for SCSS {
    fn name() -> &'static str {
        "SCSS"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(glob("web/*.scss")?.flatten())
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["generated/static/styles.css"])
    }

    fn perform(_: impl Iterator<Item = PathBuf>) -> Result<()> {
        let css = grass::from_path(
            "web/styles.scss",
            &grass::Options::default()
                .style(grass::OutputStyle::Compressed)
                .load_path("node_modules/@picocss/pico/scss/")
                .load_path("node_modules/@catppuccin/palette/scss"),
        )?;

        fs::write("generated/static/styles.css", css)?;

        Ok(())
    }
}

pub struct Images;
impl Melody for Images {
    fn name() -> &'static str {
        "Images"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(std::fs::read_dir("./content/img")?.map(|f| f.unwrap().path()))
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(Self::source()?.into_iter().map(|p| {
            let p: PathBuf = p.into();
            Path::new("./generated/img").join(p.file_name().unwrap())
        }))
    }

    fn perform(_: impl Iterator<Item = PathBuf>) -> Result<()> {
        for p in Self::source()? {
            let p: PathBuf = p.into();
            let t = Path::new("./generated/img").join(p.file_name().unwrap());

            fs::copy(p, t)?;
        }

        Ok(())
    }
}
