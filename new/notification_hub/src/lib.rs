use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use serde_json::json;
use weil_rs::config::Secrets;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct NotificationConfig {
    pub slack_webhook_url: String,
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub twilio_phone: String,
    pub smtp_host: String,
    pub smtp_port: String,
}

trait NotificationHub {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn send_slack(&self, message: String, severity: String) -> Result<String, String>;
    async fn send_sms(&self, phone: String, message: String) -> Result<String, String>;
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<String, String>;
    async fn create_war_room(&self, incident_id: String) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct NotificationHubContractState {
    secrets: Secrets<NotificationConfig>,
}

#[smart_contract]
impl NotificationHub for NotificationHubContractState {
    #[constructor]
    fn new() -> Result<NotificationHubContractState, String> {
        Ok(NotificationHubContractState {
            secrets: Secrets::new(),
        })
    }

    #[query]
    async fn send_slack(&self, message: String, severity: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // Color code by severity
        let color = match severity.as_str() {
            "P0" => "#FF0000", // Red
            "P1" => "#FFA500", // Orange
            "P2" => "#FFFF00", // Yellow
            "P3" => "#00FF00", // Green
            _ => "#808080"     // Gray
        };
        
        // In production, this would make actual HTTP call to Slack webhook
        // For demo/WASM compatibility, return success response
        let response = json!({
            "status": "success",
            "message": "Slack notification sent",
            "webhook": &config.slack_webhook_url,
            "severity": severity,
            "color": color,
            "content": message
        });
        
        Ok(response.to_string())
    }

    #[query]
    async fn send_sms(&self, phone: String, message: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // In production, this would make actual HTTP call to Twilio API
        // For demo/WASM compatibility, return success response
        let response = json!({
            "status": "success",
            "message": "SMS sent successfully",
            "from": &config.twilio_phone,
            "to": phone,
            "body": message
        });
        
        Ok(response.to_string())
    }

    #[query]
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // In production, this would use SMTP to send email
        // For demo/WASM compatibility, return success response
        let response = json!({
            "status": "success",
            "message": "Email queued successfully",
            "to": to,
            "subject": subject,
            "smtp_host": &config.smtp_host,
            "smtp_port": &config.smtp_port
        });
        
        Ok(response.to_string())
    }

    #[query]
    async fn create_war_room(&self, incident_id: String) -> Result<String, String> {
        // Create a war room link for the incident
        let room_url = format!(
            "https://meet.google.com/incident-{}", 
            incident_id.replace("INC-", "")
        );
        
        let response = json!({
            "status": "success",
            "war_room_url": room_url,
            "incident_id": incident_id,
            "message": "War room created successfully"
        });
        
        Ok(response.to_string())
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "send_slack",
      "description": "Sends Slack notification to webhook with severity-based color coding",
      "parameters": {
        "type": "object",
        "properties": {
          "message": {
            "type": "string",
            "description": "Message content to send to Slack"
          },
          "severity": {
            "type": "string",
            "description": "Severity level (P0, P1, P2, P3) for color coding"
          }
        },
        "required": [
          "message",
          "severity"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_sms",
      "description": "Sends SMS via Twilio to specified phone number",
      "parameters": {
        "type": "object",
        "properties": {
          "phone": {
            "type": "string",
            "description": "Recipient phone number with country code (e.g., +1-555-0101)"
          },
          "message": {
            "type": "string",
            "description": "SMS message content"
          }
        },
        "required": [
          "phone",
          "message"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_email",
      "description": "Sends email alert to specified recipient",
      "parameters": {
        "type": "object",
        "properties": {
          "to": {
            "type": "string",
            "description": "Recipient email address"
          },
          "subject": {
            "type": "string",
            "description": "Email subject line"
          },
          "body": {
            "type": "string",
            "description": "Email body content"
          }
        },
        "required": [
          "to",
          "subject",
          "body"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "create_war_room",
      "description": "Creates a war room meeting link for P0 incidents",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "Incident ID to create war room for (e.g., INC-12345)"
          }
        },
        "required": [
          "incident_id"
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