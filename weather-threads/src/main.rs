use std::sync::mpsc;
use std::thread;

use clap::{Arg, App};
use curl::easy::Easy;
use env_logger;
use log::{debug, info, warn};
use serde_json::{Value};

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather?units=metric&";

fn main() {
    env_logger::init();

    let apikey = include_str!("apikey").trim();
    debug!("Open Weather API Key: {:}", apikey);

    let arguments = App::new("Weather")
                            .version("0.1.0")
                            .about("Get weather for cities")
                            .arg(
                                Arg::with_name("cities")
                                    .multiple(true)
                            )
                            .get_matches();
    let cities = arguments.values_of("cities").unwrap_or_default();

    // Spawn a thread for each API call and collect API call results
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    let num_threads = cities.len();
    for city in cities {
        let c = String::from(city);
        let ntx = tx.clone();
        let h = thread::spawn( move || {
            let r = weather_call(apikey, c);
            ntx.send(r).unwrap();
        });
        handles.push(h);
    }

    for _ in 0..num_threads {
        let call_res = rx.recv().unwrap();
        println!(
            "Temperature in {:} is {:}C with {:}",
            call_res.city,
            call_res.temperature,
            call_res.weather,
        );
    }

    for h in handles {
        h.join().unwrap();
    }

}

#[derive(Debug)]
struct CallResult {
    city: String,
    weather: String,
    temperature: f64,
}

/// Fetches weather data from Open Weather service.
/// Then parses the returned JSON data and prints out the weather info
fn weather_call (apikey: &str, city_query: String) -> CallResult {
    // Create a new curl object
    let mut easy = Easy::new();

    // Set the request URL
    let url = format!("{:}q={:}&appid={:}", BASE_URL, city_query, apikey);
    match easy.url(&url) {
        Err(e) => panic!("CURL Error code: {:}", e.code()),
        _ => ()
    }

    // Create buffer to save returned data
    let mut weather_data = String::new();

    // TODO: propagate errors and log them
    // Create a transfer object to write to the buffer
    let mut transfer = easy.transfer();
    transfer.write_function(
        |data| {
            weather_data = match String::from_utf8(
                Vec::from(data)
            ) {
                Ok(x) => x,
                Err(_) => format!("Malformed data encontered while fetching weather data for {:}", city_query)
            };
            Ok(data.len())
        }).unwrap_or_else(|_| warn!("Error writing weather data {:}", city_query));
    info!("Fetching weather data...");
    transfer.perform().unwrap_or_else(|_| warn!("Error fetching weather data {:}", city_query));
    info!("...done");
    drop(transfer);

    // Parse data
    let parsed_value: Value = serde_json::from_str(&weather_data).unwrap();

    let city = String::from(
        parsed_value.get("name").unwrap()
            .as_str().unwrap()
    );
    let weather = String::from(
        parsed_value.get("weather").unwrap()
            .get(0).unwrap()
            .get("description").unwrap()
            .as_str().unwrap()
    );

    let temperature = parsed_value.get("main").unwrap()
                                    .get("temp").unwrap()
                                    .as_f64().unwrap();

    CallResult {
        city,
        weather,
        temperature,
    }
}
