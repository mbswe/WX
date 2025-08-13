use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Units {
    pub air_pressure_at_sea_level: String,
    pub air_temperature: String,
    pub cloud_area_fraction: String,
    pub precipitation_amount: String,
    pub relative_humidity: String,
    pub wind_from_direction: String,
    pub wind_speed: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Timeseries {
    pub time: String,
    pub data: Data
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub instant: Instant,
    pub next_1_hours: Option<Next1Hours>,
    pub next_6_hours: Option<Next6Hours>,
    pub next_12_hours: Option<Next12Hours>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Instant {
    pub details: Details
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Details {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_speed_of_gust: Option<f64>,
    pub ultraviolet_index_clear_sky: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Next1Hours {
    pub summary: Summary,
    pub details: Option<Details>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Summary {
    pub symbol_code: String
}


