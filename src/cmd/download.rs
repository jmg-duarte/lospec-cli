use std::{path::PathBuf, str::FromStr};

use clap::{builder::PossibleValue, ValueEnum};
use serde_json::json;

use crate::palette::Color;

#[derive(Clone, Debug)]
pub enum Format {
    /// Xcode's `.colorset` folder format
    Colorset,
    /// List of hex values
    Hex,
}

impl Format {
    fn download_file_extension(&self) -> &'static str {
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

#[derive(Debug)]
pub struct Download {
    slug: String,
    path: PathBuf,
    format: Format,
}

impl Download {
    pub fn new(slug: String, path: PathBuf, format: Format) -> Self {
        Self { slug, path, format }
    }

    pub async fn execute(self) {
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://lospec.com/palette-list/{}.{}",
                self.slug,
                self.format.download_file_extension()
            ))
            .send()
            .await
            .unwrap();

        match self.format {
            // TODO: handle already exists
            Format::Colorset => {
                let contents = response.text().await.unwrap();

                // Create the target folder
                std::fs::create_dir_all(&self.path).unwrap();

                // Create the folder's Contents.json
                std::fs::write(self.path.join("Contents.json"), generate_folder_contents())
                    .unwrap();

                for color in contents.split("\n").filter(|s| !s.is_empty()) {
                    let colorset_path = self.path.join(format!("{}.colorset", color));
                    // Prepare the folder
                    std::fs::create_dir_all(&colorset_path).unwrap();
                    // Write all colors
                    std::fs::write(
                        colorset_path.join("Contents.json"),
                        generate_contents(Color::from_str(color).unwrap()),
                    )
                    .unwrap();
                }
            }
            Format::Hex => std::fs::write(self.path, response.bytes().await.unwrap()).unwrap(),
        }
    }
}

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
