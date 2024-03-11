use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{builder::PossibleValue, ValueEnum};
use serde_json::json;
use thiserror::Error;

use crate::palette::Color;

#[derive(Clone, Debug)]
pub enum Format {
    /// Xcode's `.colorset` folder format
    Colorset,
    /// List of hex values
    Hex,
    // TODO: look into https://lospec.com/palettes/api
}

impl Format {
    /// Return the file extension used by Lospec.
    fn file_extension(&self) -> &'static str {
        match self {
            Format::Colorset | Format::Hex => "hex",
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Colorset, Self::Hex]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(match self {
            Format::Colorset => "colorset",
            Format::Hex => "hex",
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
}

impl Download {
    pub fn new(slug: String, path: PathBuf, format: Format) -> Self {
        Self { slug, path, format }
    }

    /// Execute the download request.
    pub async fn execute(self) -> Result<(), Error> {
        let client = reqwest::Client::new();
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
            Format::Hex => {
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
