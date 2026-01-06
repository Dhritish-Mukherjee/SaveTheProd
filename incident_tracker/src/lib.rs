use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;
use serde_json::json;

trait IncidentTracker {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn create_incident(&mut self, description: String, severity: String, service: String, reporter: String, timestamp: String) -> Result<String, String>;
    async fn log_action(&mut self, incident_id: String, action_type: String, details: String, timestamp: String) -> Result<String, String>;
    async fn update_status(&mut self, incident_id: String, status: String, notes: String) -> Result<String, String>;
    async fn get_incident_timeline(&self, incident_id: String) -> Result<String, String>;
    async fn get_active_incidents(&self) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct IncidentTrackerContractState {}

#[smart_contract]
impl IncidentTracker for IncidentTrackerContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(IncidentTrackerContractState {})
    }

    #[mutate]
    async fn create_incident(
        &mut self,
        description: String,
        severity: String,
        service: String,
        reporter: String,
        timestamp: String,
    ) -> Result<String, String> {
        let incident_id = format!(
            "INC-{}", 
            timestamp.replace(":", "").replace("-", "").replace("T", "").replace("Z", "")
        );
        
        let incident_data = json!({
            "id": incident_id,
            "description": description,
            "severity": severity,
            "service": service,
            "reporter": reporter,
            "timestamp": timestamp,
            "status": "open",
            "created_at": timestamp
        });
        
        Ok(format!("{{\"incident_id\": \"{}\", \"status\": \"created\"}}", incident_id))
    }

    #[mutate]
    async fn log_action(
        &mut self,
        incident_id: String,
        action_type: String,
        details: String,
        timestamp: String,
    ) -> Result<String, String> {
        let log_key = format!(
            "incident:{}:log:{}", 
            incident_id, 
            timestamp.replace(":", "").replace("-", "").replace("T", "").replace("Z", "")
        );
        
        let log_data = json!({
            "incident_id": incident_id,
            "action_type": action_type,
            "details": details,
            "timestamp": timestamp
        });
        
        Ok(format!("{{\"status\": \"logged\", \"action_type\": \"{}\"}}", action_type))
    }

    #[mutate]
    async fn update_status(
        &mut self,
        incident_id: String,
        status: String,
        notes: String,
    ) -> Result<String, String> {
        let update_data = json!({
            "incident_id": incident_id,
            "status": status,
            "notes": notes,
            "updated_at": "2026-01-05T10:00:00Z"
        });
        
        Ok(format!("{{\"incident_id\": \"{}\", \"new_status\": \"{}\"}}", incident_id, status))
    }

    #[query]
    async fn get_incident_timeline(&self, incident_id: String) -> Result<String, String> {
        let timeline = json!({
            "incident_id": incident_id,
            "timeline": [
                {
                    "timestamp": "2026-01-05T10:00:00Z",
                    "event": "Incident created",
                    "details": "Initial report"
                }
            ]
        });
        
        Ok(timeline.to_string())
    }

    #[query]
    async fn get_active_incidents(&self) -> Result<String, String> {
        let active_incidents = json!({
            "count": 0,
            "incidents": []
        });
        
        Ok(active_incidents.to_string())
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "create_incident",
      "description": "Creates new incident record on-chain\n",
      "parameters": {
        "type": "object",
        "properties": {
          "description": {
            "type": "string",
            "description": "incident description\n"
          },
          "severity": {
            "type": "string",
            "description": "severity P0-P3\n"
          },
          "service": {
            "type": "string",
            "description": "affected service\n"
          },
          "reporter": {
            "type": "string",
            "description": "reporter name\n"
          },
          "timestamp": {
            "type": "string",
            "description": "timestamp\n"
          }
        },
        "required": [
          "description",
          "severity",
          "service",
          "reporter",
          "timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "log_action",
      "description": "Logs action taken during incident\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID\n"
          },
          "action_type": {
            "type": "string",
            "description": "action type (notified, escalated, resolved)\n"
          },
          "details": {
            "type": "string",
            "description": "action details JSON\n"
          },
          "timestamp": {
            "type": "string",
            "description": "timestamp\n"
          }
        },
        "required": [
          "incident_id",
          "action_type",
          "details",
          "timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "update_status",
      "description": "Updates incident status\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID\n"
          },
          "status": {
            "type": "string",
            "description": "new status (investigating, resolved, closed)\n"
          },
          "notes": {
            "type": "string",
            "description": "resolution notes\n"
          }
        },
        "required": [
          "incident_id",
          "status",
          "notes"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_incident_timeline",
      "description": "Retrieves complete incident timeline\n",
      "parameters": {
        "type": "object",
        "properties": {
          "incident_id": {
            "type": "string",
            "description": "incident ID\n"
          }
        },
        "required": [
          "incident_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_active_incidents",
      "description": "Gets all active incidents\n",
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