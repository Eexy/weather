use std::{fmt, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, about = "A small weather cli inspired by fastfetch")]
pub struct Args {
    #[arg(short, long)]
    pub city: String,

    #[arg(long)]
    pub country: Option<Country>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Country(String);

impl FromStr for Country {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 || !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("country must be a 2-letter code (e.g. FR)".to_string());
        }

        Ok(Self(s.to_uppercase()))
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
