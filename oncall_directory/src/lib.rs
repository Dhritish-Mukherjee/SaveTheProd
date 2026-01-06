
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::config::Secrets;
use serde_json::json;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct DirectoryConfig {
    pub pagerduty_api_key: String,
    pub api_endpoint: String,
}

trait OncallDirectory {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_oncall_engineer(&self, team: String) -> Result<String, String>;
    async fn get_escalation_chain(&self, team: String, severity: String) -> Result<String, String>;
    async fn get_team_channels(&self, team: String) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct OncallDirectoryContractState {
    // define your contract state here!
    secrets: Secrets<DirectoryConfig>,
}

#[smart_contract]
impl OncallDirectory for OncallDirectoryContractState {
    #[constructor]
    fn new() -> Result<Self, String>
where
    Self: Sized,
{
    Ok(OncallDirectoryContractState {
        secrets: Secrets::new(),
    })
}


    #[query]
    async fn get_oncall_engineer(&self, team: String) -> Result<String, String> {
        let config = self.secrets.config();
        
        // In production, query PagerDuty API
        // For demo, return mock data based on team
        
        let engineer = match team.as_str() {
            "backend" => json!({
                "name": "Alice Johnson",
                "phone": "+1-555-0101",
                "email": "alice@company.com",
                "slack": "@alice"
            }),
            "frontend" => json!({
                "name": "Bob Smith",
                "phone": "+1-555-0102",
                "email": "bob@company.com",
                "slack": "@bob"
            }),
            "infra" => json!({
                "name": "Charlie Davis",
                "phone": "+1-555-0103",
                "email": "charlie@company.com",
                "slack": "@charlie"
            }),
            _ => json!({
                "name": "Default Oncall",
                "phone": "+1-555-0100",
                "email": "oncall@company.com",
                "slack": "@oncall"
            })
        };
        
        Ok(engineer.to_string())
    }

    #[query]
    async fn get_escalation_chain(&self, team: String, severity: String) -> Result<String, String> {
        let chain = match severity.as_str() {
            "P0" => json!({
                "levels": [
                    {"role": "on_call_engineer", "contact_immediately": true},
                    {"role": "team_lead", "contact_after": "5min"},
                    {"role": "vp_engineering", "contact_after": "15min"}
                ]
            }),
            "P1" => json!({
                "levels": [
                    {"role": "on_call_engineer", "contact_immediately": true},
                    {"role": "team_lead", "contact_after": "30min"}
                ]
            }),
            "P2" => json!({
                "levels": [
                    {"role": "on_call_engineer", "contact_immediately": false}
                ]
            }),
            _ => json!({
                "levels": [
                    {"role": "ticket_queue", "contact_immediately": false}
                ]
            })
        };
        
        Ok(chain.to_string())
    }

    #[query]
    async fn get_team_channels(&self, team: String) -> Result<String, String> {
        let channels = match team.as_str() {
        "backend" => json!({
            "primary": "#backend-incidents",
            "general": "#backend",
            "alerts": "#backend-alerts"
        }),
        "frontend" => json!({
            "primary": "#frontend-incidents",
            "general": "#frontend",
            "alerts": "#frontend-alerts"
        }),
        _ => json!({
            "primary": "#incidents",
            "general": "#engineering",
            "alerts": "#alerts"
        })
    };
    
    Ok(channels.to_string())
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_oncall_engineer",
      "description": "Gets current on-call engineer for a team\n",
      "parameters": {
        "type": "object",
        "properties": {
          "team": {
            "type": "string",
            "description": "team name (backend, frontend, infra)\n"
          }
        },
        "required": [
          "team"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_escalation_chain",
      "description": "Gets escalation chain for severity\n",
      "parameters": {
        "type": "object",
        "properties": {
          "team": {
            "type": "string",
            "description": "team name\n"
          },
          "severity": {
            "type": "string",
            "description": "severity level P0-P3\n"
          }
        },
        "required": [
          "team",
          "severity"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_team_channels",
      "description": "Gets team Slack channels\n",
      "parameters": {
        "type": "object",
        "properties": {
          "team": {
            "type": "string",
            "description": "team name\n"
          }
        },
        "required": [
          "team"
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

