use crate::client::{CurrentWeather, Location};

pub struct Renderer;

impl Renderer {
    pub fn list(location: &Location, weather: &CurrentWeather) -> String {
        let description = weather
            .conditions
            .first()
            .map_or("unknown", |c| c.description.as_str());

        format!(
            "- City : {}\n- Country : {}\n- Lat : {}, Lon : {}\n- Temp : {}\n- Description : {description}",
            location.name, location.country, location.lat, location.lon, weather.measurements.temp,
        )
    }
}
