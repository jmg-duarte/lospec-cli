use std::num::ParseIntError;

use serde::{de::Visitor, Deserialize};

type UtcDateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
pub struct User {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Palette {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub tags: Vec<String>,
    pub colors: Vec<Color>,
    pub title: String,
    pub slug: String,
    pub published_at: UtcDateTime,
    pub user: Option<User>,
    pub created_at: UtcDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Palettes {
    pub palettes: Vec<Palette>,
}
