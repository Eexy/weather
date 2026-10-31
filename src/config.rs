use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub city: String,
    pub country: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingCity,
    MissingCountry,
    InvalidArgument(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCity => write!(f, "argument city `-c` need a value"),
            Self::MissingCountry => write!(f, "argument country `--country` need a value"),
            Self::InvalidArgument(arg) => write!(f, "invalid argument {arg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, ConfigError> {
        args.next();
        let mut city: Option<String> = None;
        let mut country: Option<String> = None;

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "-c" | "--city" => {
                    city = Some(args.next().ok_or(ConfigError::MissingCity)?);
                }
                "--country" => {
                    country = Some(args.next().ok_or(ConfigError::MissingCountry)?);
                }
                _ => {
                    return Err(ConfigError::InvalidArgument(flag));
                }
            }
        }

        Ok(Config {
            city: city.ok_or(ConfigError::MissingCity)?,
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

        assert_eq!(Config::parse(args).err(), Some(ConfigError::MissingCity));
    }

    #[test]
    fn error_when_no_city_longer_argument() {
        let args = [String::from("program_name"), String::from("--city")].into_iter();

        assert_eq!(Config::parse(args).err(), Some(ConfigError::MissingCity));
    }

    #[test]
    fn error_when_no_country_and_flag_is_present() {
        let args = [String::from("program_name"), String::from("--country")].into_iter();

        assert_eq!(Config::parse(args).err(), Some(ConfigError::MissingCountry));
    }

    #[test]
    fn parse_valid_config_with_only_city() {
        let args = [
            String::from("program_name"),
            String::from("--city"),
            String::from("test"),
        ]
        .into_iter();

        let expected_config = Config {
            city: "test".to_string(),
            country: None,
        };

        assert_eq!(Config::parse(args), Ok(expected_config));
    }

    #[test]
    fn error_when_invalid_argument() {
        let args = [String::from("program_name"), String::from("--x")].into_iter();

        assert_eq!(
            Config::parse(args).err(),
            Some(ConfigError::InvalidArgument("--x".to_string()))
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

        let expected_config = Config {
            city: "test".to_string(),
            country: Some("test".to_string()),
        };

        assert_eq!(Config::parse(args), Ok(expected_config));
    }
}
