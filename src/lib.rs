use std::{fs, process::Command};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;
use weather::WeatherResponse;

use sys_locale::get_locale;
rust_i18n::i18n!("locales", fallback = "en");

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
    lang: String,
}

// Helper function that retrieves the current latitude and longitude using an external IP geolocation service.
fn get_current_location() -> Option<GeoLocation> {
    let response = reqwest::blocking::get("http://ip-api.com/json").ok()?;
    response.json::<GeoLocation>().ok()
}

#[init]
fn init(config_dir: RString) -> State {
    // Detect and set system language
    let lang = get_locale().unwrap_or_else(|| String::from("en-US"));
    rust_i18n::set_locale(&lang);

    State {
        config: match fs::read_to_string(format!("{}/weather.ron", config_dir)) {
            Ok(content) => Some(ron::from_str(&content).unwrap()),
            Err(why) => {
                eprintln!("Error reading Weather config file: {}", why);
                None
            }
        },
        city_id: None,
        lang,
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
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}&lang={}",
            location.lat,
            location.lon,
            config.openweatherapi_key,
            config.units.to_string(),
            state.lang
        ));
        if let Ok(response) = response {
            let data: WeatherResponse = response.json().unwrap();
            state.city_id = Some(data.id);
            vec![Match {
                title: format!("{:.1} {}", data.main.temp, config.units.unit_suffix()).into(),
                icon: ROption::RSome("weather".into()),
                use_pango: false,
                description: ROption::RSome(RString::from(format!(
                    "{}\n{}: {} {}\n{}: {} %\n{}: {} hPa\n\n{}: {}",
                    data.weather.head.description,
                    rust_i18n::t!("feels_like"),
                    data.main.feels_like,
                    config.units.unit_suffix(),
                    rust_i18n::t!("humidity"),
                    data.main.humidity,
                    rust_i18n::t!("pressure"),
                    data.main.pressure,
                    rust_i18n::t!("data_for"),
                    data.name
                ))),
                id: ROption::RNone,
            }]
            .into()
        } else {
            vec![Match {
                title: rust_i18n::t!("error_getting_weather").into(),
                icon: ROption::RSome("dialog-error".into()),
                use_pango: false,
                description: ROption::RSome(rust_i18n::t!("error_openweather_api").into()),
                id: ROption::RNone,
            }]
            .into()
        }
    } else {
        vec![Match {
            title: rust_i18n::t!("config_not_loaded").into(),
            icon: ROption::RSome("dialog-error".into()),
            use_pango: false,
            description: ROption::RSome(rust_i18n::t!("config_malformed").into()),
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
