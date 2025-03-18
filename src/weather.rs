use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Weather {
    pub location: Location,
    pub current: CurrentWeather,
    pub wind_mph: u32,
    pub wind_kph: u32,
    pub wind_degree: u32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: f32,
    pub cloud: u32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub windchill_c: f32,
    pub windchill_f: f32,
    pub heatindex_c: f32,
    pub heatindex_f: f32,
    pub dewpoint_c: f32,
    pub dewpoint_f: f32,
    pub vis_km: f32,
    pub vis_miles: f32,
    pub uv: f32,
    pub gust_mph: f32,
    pub gust_kph: f32,
}

#[derive(Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Deserialize, Clone)]
pub struct CurrentWeather {
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: bool,
    pub condition: WeatherCondition,
}

#[derive(Deserialize, Clone)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
    pub code: u32,
}
