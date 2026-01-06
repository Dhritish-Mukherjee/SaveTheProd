
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


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
        unimplemented!();
    }


    #[query]
    async fn get_oncall_engineer(&self, team: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn get_escalation_chain(&self, team: String, severity: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn get_team_channels(&self, team: String) -> Result<String, String> {
        unimplemented!();
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

