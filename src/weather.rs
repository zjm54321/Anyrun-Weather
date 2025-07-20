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
    pub _coord: Coord,
    pub weather: NonEmpty<Weather>,
    pub _base: String,
    pub main: Main,
    pub _visibility: Option<u32>,
    pub _wind: Wind,
    pub _clouds: Clouds,
    pub _dt: i64,
    pub _sys: Sys,
    pub _timezone: i32,
    pub id: i32,
    pub name: String,
    pub _cod: i32,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
    pub _lon: f32,
    pub _lat: f32,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub _id: i32,
    pub _main: String,
    pub description: String,
    pub _icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub _temp_min: f64,
    pub _temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub _sea_level: Option<i32>,
    pub _grnd_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub _speed: f64,
    pub _deg: i32,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    pub _all: i32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub r#_type: Option<i32>, // `type` is a reserved keyword, use `r#type`
    pub _id: Option<i32>,
    pub _country: String,
    pub _sunrise: i64,
    pub _sunset: i64,
}
