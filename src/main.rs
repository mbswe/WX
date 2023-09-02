use std::{env, process::exit, io::Write};
use chrono::{DateTime, NaiveDateTime, Local, TimeZone};
use reqwest::{Error, header::USER_AGENT};
mod structs;
use structs::WeatherData;
use toml::Table;
use std::str::FromStr;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let lat: f64;
    let lng: f64;

    if args.len() == 1 {
        /*
        List all locations in the config file
         */
        let locations = get_locations_config();
        
        for item in locations.iter() {
            println!("{}", item.0);
        }
    } else if args.len() == 2 {
        /*
        Fetch weather data for the given location
         */
        let locations: toml::map::Map<String, toml::Value> = get_locations_config();
        let location = locations.get(&args[1]).unwrap();

        let lat = location.get("latitude").unwrap().to_string().parse::<f64>().unwrap();
        let lng = location.get("longitude").unwrap().to_string().parse::<f64>().unwrap();

        println!("Fetching weather data for {} (latitude: {}, longitude: {})", &args[1], lat, lng);
        
        let weather_data = fetch_weather_data(lat, lng).await;

        pretty_print_weather_data(weather_data.unwrap());
    } else if args.len() > 3 {
        /*
        Too many arguments
         */
        println!("Too many arguments");
    } else {
        /*
        Fetch weather data for latitude and longitude
         */

         if let Err(e) = f64::from_str(args[1].clone().as_str()) {
            println!("Invalid latitude: {e}");
            exit(1);
        }

        if let Err(e) = f64::from_str(args[2].clone().as_str()) {
            println!("Invalid longitude: {e}");
            exit(1);
        }

        lat = args[1].clone().parse::<f64>().unwrap();
        lng = args[2].clone().parse::<f64>().unwrap();

        println!("Fetching weather data for {}, {}", lat, lng);
        let weather_data = fetch_weather_data(lat, lng).await;

        pretty_print_weather_data(weather_data.unwrap());
    }
}

fn create_locations_config(path: &std::path::Path) {
    let mut file = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut locations = "\"Effectsoft Göteborg\" = { latitude = 57.960308, longitude = 12.126554 }\n".to_string();
    locations.push_str("\"Effectsoft Halmstad\" = { latitude = 56.676086, longitude = 12.858977 }\n");

    match file.write_all(locations.as_bytes()) {
        Ok(_) => println!("Created locations config file"),
        Err(e) => panic!("{}", e),
    };
}

fn get_locations_config() -> Table {
    let path = std::path::Path::new("locations.toml");

        if path.exists() == false {
            create_locations_config(path);
        }

        let file = match std::fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        let locations: Table = file.parse().unwrap();

    return locations;
}

async fn fetch_weather_data(lat: f64, lng: f64) -> Result<WeatherData, Error> {
    let request_url = format!("https://api.met.no/weatherapi/locationforecast/2.0/complete?lat={}&lon={}", lat, lng);
    let client = reqwest::Client::new();

    let response = client.get(&request_url)
        .header(USER_AGENT, "Gurka 1.0")
        .send()
        .await?;

    let weather_data = response.json::<WeatherData>().await?;

    Ok(weather_data)
}

fn pretty_print_weather_data(weather_data: WeatherData) {
    println!("Time\t\t\tTemperature\tHumidity\tWind speed\tWind direction\tUltraviolet index");

    for item in weather_data.properties.timeseries.iter() {
        let naive = NaiveDateTime::parse_from_str(&item.time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
        let local_date_time: DateTime<Local> = Local.from_utc_datetime(&naive);

        println!("{}\t{}\t\t{}\t\t{}\t\t{}\t\t{}",
            local_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),

            if item.data.instant.details.air_temperature.is_some() { item.data.instant.details.air_temperature.unwrap() } else { 0.0 },
            if item.data.instant.details.relative_humidity.is_some() { item.data.instant.details.relative_humidity.unwrap() } else { 0.0 },
            if item.data.instant.details.wind_speed.is_some() { item.data.instant.details.wind_speed.unwrap() } else { 0.0 },
            if item.data.instant.details.wind_from_direction.is_some() { item.data.instant.details.wind_from_direction.unwrap() } else { 0.0 },
            if item.data.instant.details.ultraviolet_index_clear_sky.is_some() { item.data.instant.details.ultraviolet_index_clear_sky.unwrap() } else { 0.0 }
        );
    }
}
