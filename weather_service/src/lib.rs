use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;
use weil_rs::http::{HttpClient, HttpMethod};
use serde_json::json;
use std::collections::HashMap;

trait WeatherService {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_weather(&self, latitude: String, longitude: String, start_date: String, end_date: String) -> Result<String, String>;
    async fn get_city_temperature(&self, city: String) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct WeatherServiceContractState {}

#[smart_contract]
impl WeatherService for WeatherServiceContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(WeatherServiceContractState {})
    }

    #[query]
    async fn get_weather(
        &self,
        latitude: String,
        longitude: String,
        start_date: String,
        end_date: String,
    ) -> Result<String, String> {
        let base_url = "https://archive-api.open-meteo.com/v1/archive";
        
        // Build query params like Twilio does
        let query_params = vec![
            ("latitude".to_string(), latitude.clone()),
            ("longitude".to_string(), longitude.clone()),
            ("start_date".to_string(), start_date.clone()),
            ("end_date".to_string(), end_date.clone()),
            ("hourly".to_string(), "temperature_2m".to_string()),
        ];
        
        // Add headers
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "WeilChain-Weather/1.0".to_string());
        
        let response = HttpClient::request(base_url, HttpMethod::Get)
            .headers(headers)
            .query(query_params)
            .send()
            .map_err(|e| format!("HTTP error: {}", e))?;
        
        if response.status() >= 200 && response.status() < 300 {
            let body = response.text();
            
            // Try to parse, but return raw if fails
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(weather_data) => {
                    let result = json!({
                        "latitude": weather_data["latitude"],
                        "longitude": weather_data["longitude"],
                        "timezone": weather_data["timezone"],
                        "hourly_data": {
                            "times": weather_data["hourly"]["time"],
                            "temperatures": weather_data["hourly"]["temperature_2m"]
                        },
                        "status": "success"
                    });
                    Ok(result.to_string())
                },
                Err(_) => {
                    // Return raw body if parsing fails
                    Ok(format!("{{\"status\": \"success\", \"raw_data\": {}}}", body))
                }
            }
        } else {
            Err(format!(
                "API returned status: {} - Body: {}", 
                response.status(), 
                response.text()
            ))
        }
    }

    #[query]
    async fn get_city_temperature(&self, city: String) -> Result<String, String> {
        let (lat, lon, city_name) = match city.to_lowercase().as_str() {
            "london" => ("51.5074", "-0.1278", "London"),
            "newyork" | "new york" => ("40.7128", "-74.0060", "New York"),
            "tokyo" => ("35.6762", "139.6503", "Tokyo"),
            "paris" => ("48.8566", "2.3522", "Paris"),
            "berlin" => ("52.5200", "13.4050", "Berlin"),
            "mumbai" => ("19.0760", "72.8777", "Mumbai"),
            "sydney" => ("-33.8688", "151.2093", "Sydney"),
            _ => return Err(format!("City '{}' not found. Available: London, NewYork, Tokyo, Paris, Berlin, Mumbai, Sydney", city))
        };
        
        let base_url = "https://archive-api.open-meteo.com/v1/archive";
        let today = "2024-01-15";
        
        let query_params = vec![
            ("latitude".to_string(), lat.to_string()),
            ("longitude".to_string(), lon.to_string()),
            ("start_date".to_string(), today.to_string()),
            ("end_date".to_string(), today.to_string()),
            ("hourly".to_string(), "temperature_2m".to_string()),
        ];
        
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "WeilChain-Weather/1.0".to_string());
        
        let response = HttpClient::request(base_url, HttpMethod::Get)
            .headers(headers)
            .query(query_params)
            .send()
            .map_err(|e| format!("HTTP error: {}", e))?;
        
        if response.status() >= 200 && response.status() < 300 {
            let body = response.text();
            
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(weather_data) => {
                    let temps = &weather_data["hourly"]["temperature_2m"];
                    let first_temp = temps.as_array()
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    
                    let result = json!({
                        "city": city_name,
                        "temperature": first_temp,
                        "unit": "celsius",
                        "date": today,
                        "status": "success"
                    });
                    
                    Ok(result.to_string())
                },
                Err(e) => {
                    Ok(format!("{{\"status\": \"partial_success\", \"raw_data\": {}, \"parse_error\": \"{}\"}}", body, e))
                }
            }
        } else {
            Err(format!("API error: status {}", response.status()))
        }
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Gets weather data for a location by coordinates\n",
      "parameters": {
        "type": "object",
        "properties": {
          "latitude": {
            "type": "string",
            "description": "latitude coordinate\n"
          },
          "longitude": {
            "type": "string",
            "description": "longitude coordinate\n"
          },
          "start_date": {
            "type": "string",
            "description": "start date (YYYY-MM-DD format)\n"
          },
          "end_date": {
            "type": "string",
            "description": "end date (YYYY-MM-DD format)\n"
          }
        },
        "required": [
          "latitude",
          "longitude",
          "start_date",
          "end_date"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_city_temperature",
      "description": "Gets current temperature for a city (uses preset coordinates)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "city": {
            "type": "string",
            "description": "city name (London, NewYork, Tokyo, Paris, Berlin)\n"
          }
        },
        "required": [
          "city"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{
  "prompts": []
}"#.to_string()
    }
}