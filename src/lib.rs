use eyre::Error;
use std::fmt::{format, Display};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json;

const DARKSKY_FORECAST_URL: &str = "https://api.darksky.net/forecast/";

type UnixTime = u64;
type Temperature = f32;
type Bearing = u16;
type Speed = f32;
type UvIndex = u8;
type CloudCover = f32;
type Humidity = f32;
type Probability = f32;
type Intensity = f32;
type MoonPhase = f32;
type Pressure = f32;
type Distance = f32;
type Ozone = f32;

#[derive(Debug, Serialize, Deserialize)]
pub struct DarkskyResult {
    latitude: Decimal,
    longitude: Decimal,
    timezone: String,
    currently: Weather,
    minutely: Option<MinutelySeries>,
    hourly: HourlySeries,
    daily: DailySeries,
    alerts: Option<Vec<Alerts>>,
    flags: Flags,
    offset: i8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Weather {
    time: UnixTime,
    summary: String,
    icon: String,
    nearest_storm_distance: Option<Distance>,
    nearest_storm_bearing: Option<Bearing>,
    precip_intensity: Intensity,
    precip_probability: Probability,
    precip_type: Option<String>,
    temperature: Temperature,
    apparent_temperature: Temperature,
    dew_point: Temperature,
    humidity: Humidity,
    pressure: Pressure,
    wind_speed: Speed,
    wind_gust: Speed,
    wind_bearing: Bearing,
    cloud_cover: CloudCover,
    uv_index: UvIndex,
    visibility: Distance,
    ozone: Ozone,
}

#[derive(Debug, Serialize, Deserialize)]
struct MinutelySeries {
    summary: String,
    icon: String,
    data: Vec<Precipation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Precipation {
    time: UnixTime,
    precip_intensity: Intensity,
    precip_probability: Probability,
}

#[derive(Debug, Serialize, Deserialize)]
struct HourlySeries {
    summary: String,
    icon: String,
    data: Vec<Weather>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailySeries {
    summary: String,
    icon: String,
    data: Vec<DailyWeather>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DailyWeather {
    time: UnixTime,
    summary: String,
    icon: String,
    sunrise_time: UnixTime,
    sunset_time: UnixTime,
    moon_phase: MoonPhase,
    precip_intensity: Intensity,
    precip_intensity_max: Intensity,
    precip_intensity_max_time: Option<UnixTime>,
    precip_probability: Probability,
    precip_type: Option<String>,
    temperature_high: Temperature,
    temperature_high_time: UnixTime,
    temperature_low: Temperature,
    temperature_low_time: UnixTime,
    apparent_temperature_high: Temperature,
    apparent_temperature_high_time: UnixTime,
    apparent_temperature_low: Temperature,
    apparent_temperature_low_time: UnixTime,
    dew_point: Temperature,
    humidity: Humidity,
    pressure: Pressure,
    wind_speed: Speed,
    wind_gust: Speed,
    wind_gust_time: UnixTime,
    wind_bearing: Bearing,
    cloud_cover: CloudCover,
    uv_index: UvIndex,
    uv_index_time: UnixTime,
    visibility: Distance,
    ozone: Ozone,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alerts {
    title: String,
    time: UnixTime,
    expires: UnixTime,
    description: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Flags {
    sources: Vec<String>,
    nearest_station: Distance,
    units: String,
}

fn get_weather_icon(iconstr: &str) -> &str {
    match iconstr {
        "clear-day" => "☀️",
        "clear-night" => "🌙",
        "rain" => "🌧",
        "snow" => "🌨",
        "sleet" => "🌨",
        "wind" => "💨",
        "fog" => "🌫",
        "cloudy" => "☁️",
        "partly-cloudy-day" => "⛅️",
        "partly-cloudy-night" => "🌙",
        "hail" => "🌧",
        "thunderstorm" => "⛈",
        "tornado" => "🌪",
        _ => "",
    }
}

impl DarkskyResult {
    pub async fn new(api_key: &str, lat: Decimal, lng: Decimal) -> Result<DarkskyResult, Error> {
        let request_url = format!(
            "{}{}/{},{}",
            DARKSKY_FORECAST_URL,
            urlencoding::encode(api_key),
            lat,
            lng,
        );
        let response = reqwest::get(request_url).await?;
        let response_body = response.text().await?;
        Ok(serde_json::from_str::<DarkskyResult>(&response_body)?)
    }
    fn get_unit(&self) -> &str {
        match self.flags.units.as_str() {
            "us" => "F",
            _ => "C",
        }
    }
    fn get_current_weather_str(&self) -> String {
        format!(
            "{:.1}°{} {} {}",
            self.currently.temperature,
            self.get_unit(),
            get_weather_icon(self.currently.icon.as_str()),
            self.currently.summary
        )
    }
}

impl Display for DarkskyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_current_weather_str())
    }
}
