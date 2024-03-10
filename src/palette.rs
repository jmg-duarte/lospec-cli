use std::str::FromStr;

use serde::{de::Visitor, Deserialize};
use thiserror::Error;

type UtcDateTime = chrono::DateTime<chrono::Utc>;

#[inline(always)]
fn err_to_serde_err<E, SE>(err: E) -> SE
where
    E: std::error::Error,
    SE: serde::de::Error,
{
    serde::de::Error::custom(err.to_string())
}

#[derive(Debug, Error)]
pub enum ParseColorError {
    #[error("unable to parse hex value")]
    ParseHexError,
    #[error(transparent)]
    ParseIntError(std::num::ParseIntError),
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // HACK: declare `s` as mut so we can move the slice forwards
        let mut s = s;
        if s.starts_with("0x") {
            s = &s[2..];
        }
        if s.len() < 6 {
            return Err(ParseColorError::ParseHexError);
        }
        let red = u8::from_str_radix(&s[0..2], 16).map_err(ParseColorError::ParseIntError)?;
        let green = u8::from_str_radix(&s[2..4], 16).map_err(ParseColorError::ParseIntError)?;
        let blue = u8::from_str_radix(&s[4..6], 16).map_err(ParseColorError::ParseIntError)?;
        Ok(Color { red, green, blue })
    }
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
                Color::from_str(v).map_err(err_to_serde_err)
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
