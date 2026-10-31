use std::fmt::{self, Debug};

use crate::config::ConfigError::{InvalidArgument, MissingApiKey, MissingCity, MissingCountry};

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

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub city: String,
    pub country: Option<String>,
    pub api_key: ApiKey,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingCity,
    MissingCountry,
    InvalidArgument,
    MissingApiKey,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingCity => write!(f, "argument city `-c` need a value"),
            MissingCountry => write!(f, "argument country `--country` need a value"),
            InvalidArgument => write!(f, "invalid argument"),
            MissingApiKey => write!(f, "missing api key for open weather"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn parse(
        mut args: impl Iterator<Item = String>,
        env_vars: impl Iterator<Item = (String, String)>,
    ) -> Result<Self, ConfigError> {
        args.next();
        let mut city: Option<String> = None;
        let mut country: Option<String> = None;
        let mut api_key: Option<ApiKey> = None;

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "-c" | "--city" => {
                    city = Some(args.next().ok_or(ConfigError::MissingCity)?);
                }
                "--country" => {
                    country = Some(args.next().ok_or(ConfigError::MissingCountry)?);
                }
                _ => {
                    return Err(ConfigError::InvalidArgument);
                }
            }
        }

        for (key, value) in env_vars {
            if key == "OPENWEATHER_API_KEY" {
                api_key = Some(ApiKey(value));
            }
        }

        Ok(Config {
            city: city.ok_or(ConfigError::MissingCity)?,
            api_key: api_key.ok_or(ConfigError::MissingApiKey)?,
            country,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_when_no_city() {
        let args = [String::from("program_name")].into_iter();
        let envs = [(String::from("key"), String::from("value"))].into_iter();

        assert_eq!(
            Config::parse(args, envs).err(),
            Some(ConfigError::MissingCity)
        );
    }

    #[test]
    fn error_when_no_city_longer_argument() {
        let args = [String::from("program_name"), String::from("--city")].into_iter();
        let envs = [(String::from("key"), String::from("value"))].into_iter();

        assert_eq!(
            Config::parse(args, envs).err(),
            Some(ConfigError::MissingCity)
        );
    }

    #[test]
    fn error_when_no_country_and_flag_is_present() {
        let args = [String::from("program_name"), String::from("--country")].into_iter();
        let envs = [(String::from("key"), String::from("value"))].into_iter();

        assert_eq!(
            Config::parse(args, envs).err(),
            Some(ConfigError::MissingCountry)
        );
    }

    #[test]
    fn parse_valid_config_with_only_city() {
        let args = [
            String::from("program_name"),
            String::from("--city"),
            String::from("test"),
        ]
        .into_iter();
        let envs = [(String::from("OPENWEATHER_API_KEY"), String::from("test"))].into_iter();

        let expected_config = Config {
            city: "test".to_string(),
            country: None,
            api_key: ApiKey("test".to_string()),
        };

        assert_eq!(Config::parse(args, envs), Ok(expected_config));
    }

    #[test]
    fn error_when_invalid_argument() {
        let args = [String::from("program_name"), String::from("--x")].into_iter();
        let envs = [(String::from("key"), String::from("value"))].into_iter();

        assert_eq!(
            Config::parse(args, envs).err(),
            Some(ConfigError::InvalidArgument)
        );
    }

    #[test]
    fn parse_valid_config_with_city_and_country() {
        let args = [
            String::from("program_name"),
            String::from("--city"),
            String::from("test"),
            String::from("--country"),
            String::from("test"),
        ]
        .into_iter();
        let envs = [(String::from("OPENWEATHER_API_KEY"), String::from("test"))].into_iter();

        let expected_config = Config {
            city: "test".to_string(),
            country: Some("test".to_string()),
            api_key: ApiKey("test".to_string()),
        };

        assert_eq!(Config::parse(args, envs), Ok(expected_config));
    }
}
