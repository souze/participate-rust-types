use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub enum ReactCommand {
    React { reaction: String, duration: u32 },
    NewReaction { reaction: String },
    RemoveReaction { reaction: String },
}

impl ReactCommand {
    pub fn from_json(json: &String) -> Result<ReactCommand, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReactionList {
    pub reactions: HashMap<String, u32>,
}

impl ReactionList {
    pub fn from_json(json: &String) -> Result<ReactionList, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub secret: String,
    pub emotes: Vec<String>,
}

impl CreateChannelRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub secret: Option<String>,
}

impl LoginRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
