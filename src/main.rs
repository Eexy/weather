use std::{env, process};

use crate::config::Config;

mod config;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let config = match Config::parse(env::args(), env::vars()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    println!("Hello, world!");

    Ok(())
}
