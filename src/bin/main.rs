use clap::Parser;
use lospec_cli::{
    cli::Cli,
    cmd::{
        download::{self, Download},
        search::{self, Filter, Search},
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    DownloadError(#[from] download::Error),
    #[error(transparent)]
    SearchError(#[from] search::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

            search.execute().await?;
        }
        Cli::Download { slug, path, format } => {
            Download::new(
                slug.clone(),
                path.unwrap_or_else(|| {
                    std::env::current_dir()
                        .expect("current directory to be valid")
                        .join(slug)
                }),
                format,
            )
            .execute()
            .await?;
        }
    }

    Ok(())
}
