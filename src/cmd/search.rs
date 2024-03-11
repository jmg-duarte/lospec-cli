use colored::Colorize;
use thiserror::Error;

use crate::{cli::Sorting, palette::Palettes};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),
}

/// Color search filter.
#[derive(Clone)]
pub enum Filter {
    /// Palettes with any number of colors.
    Any,
    /// Palettes with N maximum colors.
    Max(u16),
    /// Palettes with N minimum colors.
    Min(u16),
    /// Palettes with exactly N colors.
    Exact(u16),
}

pub struct Search {
    /// Which page to show.
    pub page: u16,
    /// Search filter.
    pub filter: Filter,
    /// Search results sorting.
    pub sorting: Sorting,
    /// Search tag (empty if no tag is being searched).
    pub tag: String,
}

impl Search {
    pub fn new(page: Option<u16>, filter: Filter, sorting: Sorting, tag: Option<String>) -> Self {
        Self {
            page: page.unwrap_or(1),
            filter,
            sorting,
            tag: tag.unwrap_or("".to_string()),
        }
    }

    /// Generate query parameters for the request.
    fn to_query(self) -> Vec<(&'static str, String)> {
        let mut params = vec![
            ("page", format!("{}", self.page)),
            ("sortingType", self.sorting.to_string()),
            ("tag", self.tag),
        ];

        match self.filter {
            Filter::Any => params.push(("colorNumberFilterType", "any".to_string())),
            Filter::Max(n) => {
                params.push(("colorNumberFilterType", "max".to_string()));
                params.push(("colorNumber", format!("{}", n)));
            }
            Filter::Min(n) => {
                params.push(("colorNumberFilterType", "min".to_string()));
                params.push(("colorNumber", format!("{}", n)));
            }
            Filter::Exact(n) => {
                params.push(("colorNumberFilterType", "exact".to_string()));
                params.push(("colorNumber", format!("{}", n)));
            }
        }

        params
    }

    /// Execute the search request.
    pub async fn execute(self) -> Result<(), Error> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://lospec.com/palette-list/load")
            .query(&self.to_query())
            .send()
            .await?;

        let json: Palettes =
            serde_json::from_slice(&response.bytes().await.map_err(Error::RequestError)?)?;

        for palette in &json.palettes {
            if let Some(user) = &palette.user {
                println!("{} ({}) by {}", palette.title, palette.slug, user.name);
            } else {
                println!("{}", palette.title);
            }

            for color in &palette.colors {
                let colored_string = "  ".on_truecolor(color.red, color.green, color.blue);
                print!("{}", colored_string);
            }
            println!();
            println!();
        }

        Ok(())
    }
}
