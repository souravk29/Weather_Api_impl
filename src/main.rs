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
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent the wind description
#[derive(Deserialize, Debug)]

struct Wind {
    speed: f64,
}

// function to get the weather info from the openWeatherMap Api

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?; // "?" for handling error , "reqwest" is asynchronous library for non blocking i/o operations
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// function to display weather info, and extract weather info from the response

fn display_weather_info(response: &WeatherResponse) {
    // Extract
    let description: &String = &response.weather[0].description;
    let temp: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let speed: f64 = response.wind.speed;

    // formatting weahter info into a string

    let weather_text = format!(
        "weather in {}: {} {}
        > temp : {:.1}°C,
        > humidity : {:.1}%,
        > pressure : {:.1}hPa,
        > speed: {:.1}m/s",
        response.name,
        description,
        // function to get temperature emoji
        get_temp_emoji(temp),
        temp,
        humidity,
        pressure,
        speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

// Function to get emoji based on the temperature

fn get_temp_emoji(temp: f64) -> &'static str {
    if temp <= 0.0 {
        "❄️ "
    } else if temp > 0.0 && temp <= 10.0 {
        "🌪️"
    } else if temp > 10.0 && temp <= 20.0 {
        "🌩️"
    } else if temp > 20.0 && temp <= 30.0 {
        "⛅"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to weather info".bright_yellow());

    loop {
        // for city
        println!("{}", "Please enter the name if the city".bright_green());

        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read the input");

        let city: &str = city.trim(); // the trim() method is used to remove leading and trailing whitespace from a string slice (&str)

        // for country
        println!(
            "{}",
            "Please enter the 2-letter country code (e.g., US, IN)".bright_green()
        );

        let mut country_code = String::new();

        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read the input");

        let country_code: &str = country_code.trim();

        // Get Api key from teh site openWeatherApp
        let api_key = "8030d6d706220c8945314bdc7e69556f";

        // Calling the function to fetch the weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error {}", err);
            }
        }

        println!("Do you want to search for weather info of another city ? (yes/no)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thanks for using our software 😊");
            break;
        }
    }
}
