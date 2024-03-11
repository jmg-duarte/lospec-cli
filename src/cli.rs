use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::cmd::download::Format;

#[derive(Clone, Debug)]
pub enum Sorting {
    Default,
    AZ,
    Downloads,
    Newest,
}

impl ValueEnum for Sorting {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Default, Self::AZ, Self::Downloads, Self::Newest]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Sorting::Default => clap::builder::PossibleValue::new("default"),
            Sorting::AZ => clap::builder::PossibleValue::new("az"),
            Sorting::Downloads => clap::builder::PossibleValue::new("downloads"),
            Sorting::Newest => clap::builder::PossibleValue::new("newest"),
        })
    }
}

impl ToString for Sorting {
    fn to_string(&self) -> String {
        match self {
            Sorting::Default => "default".to_string(),
            Sorting::AZ => "alphabetical".to_string(),
            Sorting::Downloads => "downloads".to_string(),
            Sorting::Newest => "newest".to_string(),
        }
    }
}

#[derive(Debug, Parser)]
pub enum Cli {
    #[command(about = "Search for color palettes")]
    Search {
        #[arg(long, conflicts_with_all = ["min", "exact"])]
        max: Option<u16>,
        #[arg(long, conflicts_with_all = ["max", "exact"])]
        min: Option<u16>,
        #[arg(long, conflicts_with_all = ["max", "min"])]
        exact: Option<u16>,

        #[arg(short, long)]
        page: Option<u16>,

        #[arg(long, default_value_t = Sorting::Default)]
        sorting: Sorting,

        // NOTE: expand this to perform multiple searches
        #[arg(long)]
        tag: Option<String>,
    },
    #[command(about = "Download a color palette")]
    Download {
        /// The palette slug (for example: `fairydust-8`)
        slug: String,

        /// The path to download the file(s) to. Defaults to `<current_directory>/<slug>`
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// The output format
        #[arg(short, long)]
        format: Format,
    },
}
