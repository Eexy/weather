use std::fmt::Write;

use crate::client::{CurrentWeather, Location};

pub struct Renderer;

struct Row {
    label: String,
    value: String,
    label_length: usize,
}

impl Row {
    pub fn new(label: &str, value: String) -> Self {
        Self {
            label: label.to_string(),
            value,
            label_length: label.len(),
        }
    }
}

const LABEL_PADDING: usize = 3;

impl Renderer {
    pub fn list(location: &Location, weather: &CurrentWeather) -> String {
        let description = weather
            .conditions
            .first()
            .map_or("unknown", |c| c.description.as_str());

        let rows: Vec<_> = vec![
            Row::new("City", format!("{}", location.name)),
            Row::new("Country", format!("{}", location.country)),
            Row::new("Temp", format!("{:.1}°C", weather.measurements.temp)),
            Row::new(
                "Min / Max",
                format!(
                    "{:.1}C / {:.1}°C",
                    weather.measurements.temp_min, weather.measurements.temp_max
                ),
            ),
            Row::new(
                "Feels like",
                format!("{:.1}C", weather.measurements.feels_like),
            ),
            Row::new("Description", description.to_string()),
            Row::new("Humidity", format!("{}%", weather.measurements.humidity)),
        ];

        let max_label_length = rows.iter().map(|row| row.label_length).max();
        let label_size = max_label_length.map_or(LABEL_PADDING, |v| v + LABEL_PADDING);
        let mut result = String::new();

        for row in rows {
            let _ = writeln!(
                result,
                "{:<width$}: {}",
                format!("{}", row.label),
                row.value,
                width = label_size
            );
        }

        result
    }
}
