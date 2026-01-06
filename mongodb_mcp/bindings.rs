
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct MongoDBConfig {
    pub connection_string: String,
    pub database_name: String,
    pub collection_name: String,
}

trait MongoDB {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn store_name(&self, name: String) -> Result<String, String>;
    async fn get_name(&self, id: String) -> Result<String, String>;
    async fn list_names(&self) -> Result<Vec<String>, String>;
    async fn delete_name(&self, id: String) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct MongoDBContractState {
    // define your contract state here!
    secrets: Secrets<MongoDBConfig>,
}

#[smart_contract]
impl MongoDB for MongoDBContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn store_name(&self, name: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn get_name(&self, id: String) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn list_names(&self) -> Result<Vec<String>, String> {
        unimplemented!();
    }

    #[query]
    async fn delete_name(&self, id: String) -> Result<String, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "store_name",
      "description": "Stores a name in MongoDB\n",
      "parameters": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "name"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_name",
      "description": "Retrieves a name by ID\n",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "list_names",
      "description": "Lists all stored names\n",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "delete_name",
      "description": "Deletes a name by ID\n",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "id"
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

