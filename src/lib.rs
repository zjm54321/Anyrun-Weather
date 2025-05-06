use std::{fs, process::Command};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;
use weather::WeatherResponse;

mod weather;

// Updated configuration now omits a fixed weather location.
#[derive(Debug, Deserialize)]
struct Config {
    use_ip_location: bool,
    prefix: String,
    weather_location: GeoLocation,
    openweatherapi_key: String,
    units: weather::WeatherUnits,
}

// Structure for the geolocation response from the IP API.
#[derive(Debug, Deserialize, Clone, Copy)]
struct GeoLocation {
    lat: f64,
    lon: f64,
    // Other fields from the API are omitted as we only need lat and lon.
}

pub struct State {
    config: Option<Config>,
    city_id: Option<i32>,
}

// Helper function that retrieves the current latitude and longitude using an external IP geolocation service.
fn get_current_location() -> Option<GeoLocation> {
    let response = reqwest::blocking::get("http://ip-api.com/json").ok()?;
    response.json::<GeoLocation>().ok()
}

#[init]
fn init(config_dir: RString) -> State {
    State {
        config: match fs::read_to_string(format!("{}/weather.ron", config_dir)) {
            Ok(content) => Some(ron::from_str(&content).unwrap()),
            Err(why) => {
                eprintln!("Error reading Weather config file: {}", why);
                None
            }
        },
        city_id: None,
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Weather".into(),
        icon: "weather".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString, state: &mut State) -> RVec<Match> {
    let prefix = if let Some(config) = state.config.as_ref() {
        config.prefix.as_str()
    } else {
        "wttr"
    };
    let _input = if let Some(input) = input.strip_prefix(prefix) {
        input.trim()
    } else {
        return RVec::new();
    };

    if let Some(config) = &state.config {
        let location = if config.use_ip_location {
            if let Some(geo) = get_current_location() {
                geo
            } else {
                config.weather_location
            }
        } else {
            config.weather_location
        };
        let response = reqwest::blocking::get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}",
            location.lat,
            location.lon,
            config.openweatherapi_key,
            config.units.to_string()
        ));
        if let Ok(response) = response {
            let data: WeatherResponse = response.json().unwrap();
            state.city_id = Some(data.id);
            vec![Match {
                title: format!("{:.1} {}", data.main.temp, config.units.unitSuffix()).into(),
                icon: ROption::RSome("weather".into()),
                use_pango: false,
                description: ROption::RSome(RString::from(format!(
                    "{}\nFeels like: {} {}\nHumidity: {} %\nPressure: {} hPa\n\nData for: {}",
                    data.weather.head.description,
                    data.main.feels_like,
                    config.units.unitSuffix(),
                    data.main.humidity,
                    data.main.pressure,
                    data.name
                ))),
                id: ROption::RNone,
            }]
            .into()
        } else {
            vec![Match {
                title: "Error getting weather".into(),
                icon: ROption::RSome("dialog-error".into()),
                use_pango: false,
                description: ROption::RSome("Error getting response from OpenWeatherAPI".into()),
                id: ROption::RNone,
            }]
            .into()
        }
    } else {
        vec![Match {
            title: "Weather config not loaded".into(),
            icon: ROption::RSome("dialog-error".into()),
            use_pango: false,
            description: ROption::RSome("Config either malformed or not created".into()),
            id: ROption::RNone,
        }]
        .into()
    }
}

#[handler]
fn handler(_selection: Match, state: &State) -> HandleResult {
    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xdg-open https://openweathermap.org/city/{}",
            state.city_id.unwrap()
        ))
        .spawn()
    {
        println!("Failed to open OpenWeatherMap: {}", why);
    }

    HandleResult::Close
}
