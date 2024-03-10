use colored::Colorize;

use crate::{cli::Sorting, palette::Palettes};

#[derive(Clone)]
pub enum Filter {
    Any,
    Max(u16),
    Min(u16),
    Exact(u16),
}

pub struct Search {
    pub page: u16,
    pub filter: Filter,
    pub sorting: Sorting,
    pub tag: String,
}

impl Search {
    pub fn to_query(self) -> Vec<(&'static str, String)> {
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

    pub async fn execute(self) {
        let client = reqwest::Client::new();

        let response = client
            .get("https://lospec.com/palette-list/load")
            .query(&self.to_query())
            .send()
            .await
            .unwrap();

        let json: Palettes = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

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
    }
}
