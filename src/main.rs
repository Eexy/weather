use std::{env, process};

use crate::client::WeatherClient;
use crate::config::Config;

mod client;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let config = match Config::parse(env::args()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let client = match WeatherClient::parse(env::vars()) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    client.get_coordinate(&config.city, config.country)?;

    Ok(())
}
