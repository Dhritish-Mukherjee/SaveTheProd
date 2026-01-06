
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;
use weil_rs::http::{HttpClient, HttpMethod};


trait JokeTest {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_joke(&self) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct JokeTestContractState {
    // define your contract state here!
}

#[smart_contract]
impl JokeTest for JokeTestContractState {
     #[constructor]
    fn new() -> Result<Self, String> {
        Ok(Self {})
    }


    #[query]
    async fn get_joke(&self) -> Result<String, String> {
        let url = "https://official-joke-api.appspot.com/random_joke";
        
        let response = HttpClient::request(url, HttpMethod::Get)
            .send()
            .map_err(|e| format!("HTTP error: {}", e))?;
        
        if response.status() >= 200 && response.status() < 300 {
            Ok(response.text())
        } else {
            Err(format!("Failed: HTTP {}", response.status()))
        }
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_joke",
      "description": "",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
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

