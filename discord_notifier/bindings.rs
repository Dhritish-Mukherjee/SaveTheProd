
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct DiscordConfig {
    pub webhook_url: String,
}

trait DiscordNotifier {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn send_message(&self, content: String, username: String) -> Result<String, String>;
    async fn send_incident_alert(&self, incident_id: String, severity: String, description: String, service: String) -> Result<String, String>;
    async fn send_status_update(&self, incident_id: String, status: String, message: String) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DiscordNotifierContractState {
    // define your contract state here!
    secrets: Secrets<DiscordConfig>,
}

#[smart_contract]
impl DiscordNotifier for DiscordNotifierContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn send_message(&self, content: String, username: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn send_incident_alert(&self, incident_id: String, severity: String, description: String, service: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn send_status_update(&self, incident_id: String, status: String, message: String) -> Result<String, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "send_message",
      "description": "Sends a message to Discord channel\n",
      "parameters": {
        "type": "object",
        "properties": {
          "content": {
            "type": "string",
            "description": "the message content to send\n"
          },
          "username": {
            "type": "string",
            "description": "optional username to display (default: Incident Bot)\n"
          }
        },
        "required": [
          "content",
          "username"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_incident_alert",
      "description": "Sends a formatted incident alert to Discord\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID\n"
          },
          "severity": {
            "type": "string",
            "description": "incident severity P0-P3\n"
          },
          "description": {
            "type": "string",
            "description": "incident description\n"
          },
          "service": {
            "type": "string",
            "description": "affected service name\n"
          }
        },
        "required": [
          "incident_id",
          "severity",
          "description",
          "service"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_status_update",
      "description": "Sends a status update to Discord\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID\n"
          },
          "status": {
            "type": "string",
            "description": "new status\n"
          },
          "message": {
            "type": "string",
            "description": "update message\n"
          }
        },
        "required": [
          "incident_id",
          "status",
          "message"
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

