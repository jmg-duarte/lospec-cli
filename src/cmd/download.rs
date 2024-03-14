use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{builder::PossibleValue, ValueEnum};
use serde_json::json;
use thiserror::Error;

use crate::palette::Color;

#[derive(Clone, Debug)]
pub enum PngSize {
    /// 1x1px
    X1,
    /// 8x8px
    X8,
    /// 32x32px
    X32,
}

impl PngSize {
    fn slug(&self) -> &'static str {
        match self {
            PngSize::X1 => "-1x",
            PngSize::X8 => "-8x",
            PngSize::X32 => "-32x",
        }
    }
}

impl ValueEnum for PngSize {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::X1, Self::X8, Self::X32]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::X1 => "x1",
            Self::X8 => "x8",
            Self::X32 => "x32",
        }))
    }
}

#[derive(Clone, Debug)]
pub enum Format {
    /// Xcode's `.colorset` folder format
    Colorset,
    /// List of hex values
    Hex,
    /// PNG image
    Png,
    /// JASC Pal file
    Pal,
    /// Photoshop ASE file
    Ase,
    /// Paint.NET TXT file
    Txt,
    /// GIMP GPL file
    Gpl,
}

impl Format {
    /// Return the file extension used by Lospec.
    fn file_extension(&self) -> &'static str {
        match self {
            Format::Colorset | Format::Hex => "hex",
            Format::Png => "png",
            Format::Pal => "pal",
            Format::Ase => "ase",
            Format::Txt => "txt",
            Format::Gpl => "gpl",
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Colorset,
            Self::Hex,
            Self::Png,
            Self::Pal,
            Self::Ase,
            Self::Txt,
            Self::Gpl,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::Colorset => "colorset",
            Self::Hex => "hex",
            Self::Png => "png",
            Self::Pal => "pal",
            Self::Ase => "ase",
            Self::Txt => "txt",
            Self::Gpl => "gpl",
        }))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct Download {
    /// Palette slug.
    slug: String,
    /// Output file path.
    path: PathBuf,
    /// Output file format.
    format: Format,
    /// Output file size (only if PNG).
    size: Option<PngSize>,
}

impl Download {
    pub fn new(slug: String, path: PathBuf, format: Format, size: Option<PngSize>) -> Self {
        Self {
            slug,
            path,
            format,
            size,
        }
    }

    /// Execute the download request.
    pub async fn execute(mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();

        match (&self.format, &self.size) {
            (Format::Png, None) => self.slug.push_str(PngSize::X32.slug()),
            (Format::Png, Some(size)) => self.slug.push_str(size.slug()),
            _ => {}
        }

        let response = client
            .get(format!(
                "https://lospec.com/palette-list/{}.{}",
                self.slug,
                self.format.file_extension()
            ))
            .send()
            .await?;

        match self.format {
            Format::Colorset => {
                let contents = response.text().await?;
                let colors = contents.split("\n").filter(|s| !s.is_empty());
                export_colorset(self.path, colors).map_err(Error::IoError)
            }
            Format::Hex | Format::Ase | Format::Gpl | Format::Pal | Format::Png | Format::Txt => {
                std::fs::write(self.path, response.bytes().await?).map_err(Error::IoError)
            }
        }
    }
}

/// Export a pallete as `.colorset`.
fn export_colorset<'a, P, I>(path: P, colors: I) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
    I: Iterator<Item = &'a str>,
{
    // Create the target folder
    std::fs::create_dir_all(&path)?;

    // Create the folder's Contents.json
    std::fs::write(
        path.as_ref().join("Contents.json"),
        generate_folder_contents(),
    )?;

    for color in colors {
        let colorset_path = path.as_ref().join(format!("{}.colorset", color));
        // Prepare the folder
        std::fs::create_dir_all(&colorset_path)?;
        // Write all colors
        std::fs::write(
            colorset_path.join("Contents.json"),
            generate_contents(Color::from_str(color).unwrap()),
        )?;
    }

    Ok(())
}

/// Generate the `Contents.json` for the palette folder.
fn generate_folder_contents() -> String {
    let contents_json = json! {
        {
            "info" : {
              "author" : "xcode",
              "version" : 1
            }
          }
    };
    serde_json::to_string_pretty(&contents_json).expect("json should be valid")
}

/// Generate the `Contents.json` for the `.colorset` folder.
fn generate_contents(color: Color) -> String {
    let contents_json = json! {
        {
            "colors": [
                {
                    "color": {
                        "color-space": "srgb",
                        "components": {
                            "alpha": "1.000",
                            "blue": format!("{:#X}", color.blue),
                            "green": format!("{:#X}", color.green),
                            "red": format!("{:#X}", color.red)
                        }
                    },
                    "idiom": "universal"
                }
            ],
            "info": {
                "author": "xcode",
                "version": 1
            }
        }
    };
    serde_json::to_string_pretty(&contents_json).expect("json should be valid")
}
