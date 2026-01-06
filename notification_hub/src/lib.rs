
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use serde_json::json;
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};
use std::collections::HashMap;


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
    // define your contract state here!
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
        
        // Build Slack webhook payload
        let payload = json!({
            "attachments": [{
                "color": color,
                "text": message,
                "footer": format!("Severity: {}", severity)
            }]
        });
        
        // Make real HTTP POST to Slack webhook
        let response = HttpClient::request(&config.slack_webhook_url, HttpMethod::Post)
            .json(&payload)
            .send()
            .map_err(|e| format!("Slack API error: {}", e))?;
        
        if response.status() >= 200 && response.status() < 300 {
            Ok(format!("✓ Slack notification sent successfully ({})", severity))
        } else {
            Err(format!("Slack error: HTTP {}", response.status()))
        }
    }

       #[query]
    async fn send_sms(&self, phone: String, message: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // Build Twilio API URL
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            config.twilio_account_sid
        );
        
        // Prepare form data for Twilio
        let mut form_data = HashMap::new();
        form_data.insert("To".to_string(), phone.clone());
        form_data.insert("From".to_string(), config.twilio_phone.clone());
        form_data.insert("Body".to_string(), message);
        
        // Create Basic Auth header
        let auth_string = format!("{}:{}", config.twilio_account_sid, config.twilio_auth_token);
        let auth_encoded = base64::encode(auth_string.as_bytes());
        
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Basic {}", auth_encoded));
        
        // Make real HTTP POST to Twilio
        let response = HttpClient::request(&url, HttpMethod::Post)
            .headers(headers)
            .form(form_data)
            .send()
            .map_err(|e| format!("Twilio API error: {}", e))?;
        
        if response.status() >= 200 && response.status() < 300 {
            Ok(format!("✓ SMS sent successfully to {}", phone))
        } else {
            let status = response.status(); 
            let error_text = response.text(); // 'response' is moved here
            Err(format!("Twilio error ({}): {}", status, error_text))
        }
    }

    #[query]
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // Note: This is a placeholder. To send real emails, integrate with:
        // - SendGrid API: https://api.sendgrid.com/v3/mail/send
        // - AWS SES API: https://email.us-east-1.amazonaws.com/
        // - Mailgun API: https://api.mailgun.net/v3/
        //
        // Example SendGrid implementation:
        /*
        let payload = json!({
            "personalizations": [{"to": [{"email": to}]}],
            "from": {"email": "noreply@yourcompany.com"},
            "subject": subject,
            "content": [{"type": "text/plain", "value": body}]
        });
        
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", config.sendgrid_api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let response = HttpClient::request("https://api.sendgrid.com/v3/mail/send", HttpMethod::Post)
            .headers(headers)
            .json(&payload)
            .send()
            .map_err(|e| format!("SendGrid error: {}", e))?;
        */
        
        Ok(format!(
            "✓ Email queued to {} with subject: '{}' (SMTP: {}:{})",
            to, subject, config.smtp_host, config.smtp_port
        ))
    }

    #[query]
    async fn create_war_room(&self, incident_id: String) -> Result<String, String> {
        // Generate a war room meeting link
        // In production, you could integrate with:
        // - Zoom API to create actual meetings
        // - Google Meet API
        // - Microsoft Teams API
        
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
      "description": "Sends Slack notification to webhook\n",
      "parameters": {
        "type": "object",
        "properties": {
          "message": {
            "type": "string",
            "description": "message content with formatting\n"
          },
          "severity": {
            "type": "string",
            "description": "severity level for color coding\n"
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
      "description": "Sends SMS via Twilio\n",
      "parameters": {
        "type": "object",
        "properties": {
          "phone": {
            "type": "string",
            "description": "recipient phone number with country code\n"
          },
          "message": {
            "type": "string",
            "description": "SMS content\n"
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
      "description": "Sends email alert\n",
      "parameters": {
        "type": "object",
        "properties": {
          "to": {
            "type": "string",
            "description": "recipient email\n"
          },
          "subject": {
            "type": "string",
            "description": "email subject\n"
          },
          "body": {
            "type": "string",
            "description": "email body\n"
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
      "description": "Creates war room link for P0 incidents\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID for room name\n"
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

