
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


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
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn send_slack(&self, message: String, severity: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn send_sms(&self, phone: String, message: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn send_email(&self, to: String, subject: String, body: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn create_war_room(&self, incident_id: String) -> Result<String, String> {
        unimplemented!();
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

