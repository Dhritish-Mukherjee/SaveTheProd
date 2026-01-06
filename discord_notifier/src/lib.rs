
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::webserver::WebServer;
use weil_rs::http::{HttpClient, HttpMethod};
use weil_rs::config::Secrets;
use std::collections::HashMap;
use serde_json::json;

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
        Ok(DiscordNotifierContractState {
            secrets: Secrets::new(),
        })
    }


#[query]
async fn send_message(&self, content: String, username: String) -> Result<String, String> {
    let config = self.secrets.config();
    
    // Use default username if empty
    let bot_name = if username.is_empty() {
        "Incident Bot".to_string()
    } else {
        username
    };
    
    // Create Discord message payload
    let payload = json!({
        "content": content,
        "username": bot_name
    });
    
    // Send HTTP POST to Discord webhook
    let response = HttpClient::request(&config.webhook_url, HttpMethod::Post)
        .json(&payload)
        .send()
        .map_err(|e| format!("Discord API error: {}", e))?;
    
    // Check if successful
    if response.status() >= 200 && response.status() < 300 {
        Ok(json!({
            "status": "sent",
            "message": "Message delivered to Discord"
        }).to_string())
    } else {
        Err(format!("Discord returned status code: {}", response.status()))
    }
}

#[query]
async fn send_incident_alert(
    &self,
    incident_id: String,
    severity: String,
    description: String,
    service: String,
) -> Result<String, String> {
    let config = self.secrets.config();
    
    // Map severity to Discord embed colors (decimal format)
    let color = match severity.as_str() {
        "P0" => 16711680,  // Red (#FF0000)
        "P1" => 16753920,  // Orange (#FFA500)
        "P2" => 16776960,  // Yellow (#FFFF00)
        "P3" => 65280,     // Green (#00FF00)
        _ => 8421504       // Gray (#808080)
    };
    
    // Get current timestamp
    //let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Create rich embed for Discord
    let payload = json!({
        "username": "Incident Alert System",
        "embeds": [{
            "title": format!("🚨 {} Incident: {}", severity, incident_id),
            "description": description,
            "color": color,
            "fields": [
                {
                    "name": "Severity",
                    "value": severity,
                    "inline": true
                },
                {
                    "name": "Service",
                    "value": service,
                    "inline": true
                },
                {
                    "name": "Incident ID",
                    "value": incident_id,
                    "inline": true
                }
            ],
            "footer": {
                "text": "Incident Response System"
            },
            "timestamp": "not using chrono"
        }]
    });
    
    // Send to Discord
    let response = HttpClient::request(&config.webhook_url, HttpMethod::Post)
        .json(&payload)
        .send()
        .map_err(|e| format!("Discord API error: {}", e))?;
    
    if response.status() >= 200 && response.status() < 300 {
        Ok(json!({
            "status": "sent",
            "incident_id": incident_id,
            "severity": severity
        }).to_string())
    } else {
        Err(format!("Discord webhook failed with status: {}", response.status()))
    }
}

#[query]
async fn send_status_update(
    &self,
    incident_id: String,
    status: String,
    message: String,
) -> Result<String, String> {
    let config = self.secrets.config();
    
    // Map status to colors
    let color = match status.as_str() {
        "investigating" => 16776960,  // Yellow
        "resolved" => 65280,          // Green
        "closed" => 8421504,          // Gray
        _ => 3447003                  // Blue
    };
    
    // Choose emoji based on status
    let emoji = match status.as_str() {
        "investigating" => "🔍",
        "resolved" => "✅",
        "closed" => "🔒",
        _ => "📝"
    };
    
    //let timestamp = chrono::Utc::now().to_rfc3339();
    
    let payload = json!({
        "username": "Incident Updates",
        "embeds": [{
            "title": format!("{} Status Update: {}", emoji, incident_id),
            "description": message,
            "color": color,
            "fields": [
                {
                    "name": "New Status",
                    "value": status.to_uppercase(),
                    "inline": true
                },
                {
                    "name": "Incident ID",
                    "value": incident_id,
                    "inline": true
                }
            ],
            "timestamp": "not using chrono timestamp"
        }]
    });
    
    let response = HttpClient::request(&config.webhook_url, HttpMethod::Post)
        .json(&payload)
        .send()
        .map_err(|e| format!("Discord API error: {}", e))?;
    
    if response.status() >= 200 && response.status() < 300 {
        Ok(json!({
            "status": "sent",
            "update_type": status
        }).to_string())
    } else {
        Err(format!("Failed to send status update: HTTP {}", response.status()))
    }
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

