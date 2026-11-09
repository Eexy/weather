use std::{env, process};

use clap::Parser;

use crate::{args::Args, client::WeatherClient, renderer::Renderer};

mod args;
mod client;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dotenvy::dotenv()?;
    let client = match WeatherClient::parse(env::vars()) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let location = match client.get_coordinate(&args.city, args.country) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        Ok(location) => location,
    };

    let weather = match client.get_weather(&location) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        Ok(weather) => weather,
    };

    let output = Renderer::list(&location, &weather);

    println!("{output}");
    Ok(())
}
