
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


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
pub struct WeatherServiceContractState {
    // define your contract state here!
}

#[smart_contract]
impl WeatherService for WeatherServiceContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn get_weather(&self, latitude: String, longitude: String, start_date: String, end_date: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn get_city_temperature(&self, city: String) -> Result<String, String> {
        unimplemented!();
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

