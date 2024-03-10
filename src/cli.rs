use clap::{Parser, ValueEnum};

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

#[derive(Parser)]
pub struct Cli {
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
}

#[derive(Clone)]
pub enum Filter {
    Any,
    Max(u16),
    Min(u16),
    Exact(u16),
}

pub struct Args {
    pub page: u16,
    pub filter: Filter,
    pub sorting: Sorting,
    pub tag: String,
}

impl Args {
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
}

impl From<Cli> for Args {
    fn from(value: Cli) -> Self {
        let filter = if let Some(max) = value.max {
            Filter::Max(max)
        } else if let Some(min) = value.min {
            Filter::Min(min)
        } else if let Some(exact) = value.exact {
            Filter::Exact(exact)
        } else {
            Filter::Any
        };

        Args {
            filter,
            page: value.page.unwrap_or(1),
            sorting: value.sorting,
            tag: value.tag.unwrap_or("".to_string()),
        }
    }
}
