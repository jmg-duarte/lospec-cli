use std::num::ParseIntError;

use colored::Colorize;
use serde::{de::Visitor, Deserialize};

type UtcDateTime = chrono::DateTime<chrono::Utc>;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let response = client
        .get("https://lospec.com/palette-list/load")
        .query(&[
            ("colorNumberFilterType", "any"),
            ("colorNumber", "8"),
            ("page", "1"),
            ("sortingType", "default"),
            ("tag", ""),
        ])
        .send()
        .await
        .unwrap();

    let json: Palettes = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    for palette in &json.palettes {
        println!("{} by {}", palette.title, palette.user.name);
        for color in &palette.colors {
            let colored_string = "  ".on_truecolor(color.red, color.green, color.blue);
            print!("{}", colored_string);
        }
        println!();
        println!();
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hex value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                fn parse_int_to_serde_err<E: serde::de::Error>(err: ParseIntError) -> E {
                    serde::de::Error::custom(err.to_string())
                }

                let red = u8::from_str_radix(&v[0..2], 16).map_err(parse_int_to_serde_err)?;
                let green = u8::from_str_radix(&v[2..4], 16).map_err(parse_int_to_serde_err)?;
                let blue = u8::from_str_radix(&v[4..6], 16).map_err(parse_int_to_serde_err)?;
                Ok(Color { red, green, blue })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Palette {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    tags: Vec<String>,
    colors: Vec<Color>, // TODO: convert
    title: String,
    slug: String,
    published_at: UtcDateTime, // TODO: convert
    user: User,
    created_at: UtcDateTime,
}

#[derive(Debug, Deserialize)]
struct Palettes {
    palettes: Vec<Palette>,
}
