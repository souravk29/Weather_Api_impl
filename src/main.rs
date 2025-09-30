use colored::*;
use serde::Deserialize;
use std::io;

// Struct to Deserialize the JSON response from the openWeatherMap API

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent the weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent the wind description
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/3.0/onecall?q={},{}&units=metrics&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?; // "?" for handling error 
}
