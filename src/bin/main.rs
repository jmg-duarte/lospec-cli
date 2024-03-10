use clap::Parser;
use lospec_cli::{
    cli::Cli,
    cmd::{
        download::Download,
        search::{Filter, Search},
    },
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Search {
            max,
            min,
            exact,
            page,
            sorting,
            tag,
        } => {
            let filter = if let Some(max) = max {
                Filter::Max(max)
            } else if let Some(min) = min {
                Filter::Min(min)
            } else if let Some(exact) = exact {
                Filter::Exact(exact)
            } else {
                Filter::Any
            };

            let search = Search {
                filter,
                sorting,
                page: page.unwrap_or(1),
                tag: tag.unwrap_or("".to_string()),
            };

            search.execute().await
        }
        Cli::Download { slug, path, format } => Download::new(slug, path, format).execute().await,
    }
}
