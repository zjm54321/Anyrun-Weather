# Anyrun-Weather
## In action:
![image](https://github.com/user-attachments/assets/20989d38-2682-454d-b2e8-c3e4cdcd4a8d)

## Usage:
### Compilation:
Compile with `cargo build --release` and copy the resulting .so file from `target/release/libanyrun_weather.so` to `~/.config/anyrun/plugins`  
### Configuration:
Write the config in the following fashion:  

**Config file:**  
`~/.config/anyrun/weather.ron`  

**Config format:**  
```
Config(
  use_ip_location: {true|false},
  prefix: "{your preferred prefix}",
  weather_location: GeoLocation(
    lon: {longitude for your location}, 
    lat: {latitude for your location}
  ),
  openweatherapi_key: "{your OpenWeatherApi key}",
  units: {your preferred units: Metric|Imperial|Standard}
)
```

**Example:**

```
Config(
  use_ip_location: false,
  prefix: "wttr",
  weather_location: Coord(
    lon: 0.0, 
    lat: 0.0
  ),
  openweatherapi_key: "{removed for privacy}",
  units: Metric
)

```
