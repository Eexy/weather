use std::fmt::{self, Debug};

use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;

use crate::args::Country;

#[derive(PartialEq, Eq)]
pub struct ApiKey(String);

impl Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKey: *********")
    }
}

impl ApiKey {
    pub fn expose(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum ClientError {
    MissingApiKey,
    Request(reqwest::Error),
    InvalidApiKey,
    UnexpectedStatus(reqwest::StatusCode),
    InvalidResponse(reqwest::Error),
    CityNotFound(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingApiKey => write!(f, "OPENWEATHER_API_KEY env key is not set"),
            Self::Request(e) => write!(f, "could no reach OpenWeather: {e}"),
            Self::InvalidApiKey => write!(f, "OpenWeather rejected the api key"),
            Self::UnexpectedStatus(status) => write!(f, "OpenWeather rejected {status}"),
            Self::InvalidResponse(e) => write!(f, "unecpected response from OpenWeather: {e}"),
            Self::CityNotFound(city) => write!(f, "no city found name `{city}`"),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    id: u16,
    main: String,
    pub description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Measurements {
    pub temp: f32,
    temp_min: f32,
    temp_max: f32,
    feels_like: f32,
    pressure: u16,
    humidity: u16,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    #[serde(rename = "main")]
    pub measurements: Measurements,
    #[serde(rename = "weather")]
    pub conditions: Vec<Condition>,
}

#[derive(Debug)]
pub struct WeatherClient {
    pub api_key: ApiKey,
    pub client: Client,
}

impl WeatherClient {
    pub fn parse(env_vars: impl Iterator<Item = (String, String)>) -> Result<Self, ClientError> {
        let mut api_key: Option<ApiKey> = None;

        for (key, value) in env_vars {
            if key == "OPENWEATHER_API_KEY" {
                api_key = Some(ApiKey(value));
            }
        }

        let client = reqwest::blocking::Client::new();

        Ok(WeatherClient {
            api_key: api_key.ok_or(ClientError::MissingApiKey)?,
            client,
        })
    }

    pub fn get_coordinate(
        &self,
        city: &str,
        country: Option<Country>,
    ) -> Result<Location, ClientError> {
        let url = "http://api.openweathermap.org/geo/1.0/direct";

        let query = match country {
            Some(country) => format!("{city},{country}"),
            None => city.to_string(),
        };

        let response = self
            .client
            .get(url)
            .query(&[
                ("q", query.as_str()),
                ("limit", "1"),
                ("appid", self.api_key.expose()),
            ])
            .send()
            .map_err(ClientError::Request)?;

        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => return Err(ClientError::InvalidApiKey),
            status if !status.is_success() => return Err(ClientError::UnexpectedStatus(status)),
            _ => {}
        }

        let locations: Vec<Location> = response.json().map_err(ClientError::InvalidResponse)?;

        locations
            .into_iter()
            .next()
            .ok_or_else(|| ClientError::CityNotFound(city.to_string()))
    }

    pub fn get_weather(&self, location: &Location) -> Result<CurrentWeather, ClientError> {
        let url = "https://api.openweathermap.org/data/2.5/weather";

        let response = self
            .client
            .get(url)
            .query(&[
                ("lat", location.lat.to_string().as_str()),
                ("lon", location.lon.to_string().as_str()),
                ("units", "metrics"),
                ("appid", self.api_key.expose()),
            ])
            .send()
            .map_err(ClientError::Request)?;

        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => return Err(ClientError::InvalidApiKey),
            status if !status.is_success() => return Err(ClientError::UnexpectedStatus(status)),
            _ => {}
        }

        let weather: CurrentWeather = response.json().map_err(ClientError::InvalidResponse)?;

        Ok(weather)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_when_missing_api_key() {
        let env = [].into_iter();

        assert!(matches!(
            WeatherClient::parse(env),
            Err(ClientError::MissingApiKey)
        ));
    }

    #[test]
    fn parse_valid_key() {
        let envs = [(String::from("OPENWEATHER_API_KEY"), String::from("test"))].into_iter();

        assert!(
            matches!(WeatherClient::parse(envs), Ok(client) if client.api_key.expose() == "test")
        );
    }
}
