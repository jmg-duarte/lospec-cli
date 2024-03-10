use clap::Parser;
use colored::Colorize;
use lospec_cli::cli::{Args, Cli};

#[tokio::main]
async fn main() {
    let args = Args::from(Cli::parse());

    let client = reqwest::Client::new();

    let response = client
        .get("https://lospec.com/palette-list/load")
        .query(&args.to_query())
        .send()
        .await
        .unwrap();

    let json: lospec_cli::palette::Palettes =
        serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    for palette in &json.palettes {
        if let Some(user) = &palette.user {
            println!("{} by {}", palette.title, user.name);
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
