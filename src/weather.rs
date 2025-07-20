#![allow(unused)]

use nonempty::NonEmpty;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub enum WeatherUnits {
    Standard,
    #[default]
    Metric,
    Imperial,
}

impl std::string::ToString for WeatherUnits {
    fn to_string(&self) -> String {
        match self {
            WeatherUnits::Standard => "standard".into(),
            WeatherUnits::Metric => "metric".into(),
            WeatherUnits::Imperial => "imperial".into(),
        }
    }
}

impl WeatherUnits {
    pub fn unit_suffix(&self) -> String {
        match self {
            WeatherUnits::Standard => "K".into(),
            WeatherUnits::Metric => "°C".into(),
            WeatherUnits::Imperial => "°F".into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: NonEmpty<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub r#type: Option<i32>, // `type` is a reserved keyword, use `r#type`
    pub id: Option<i32>,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
