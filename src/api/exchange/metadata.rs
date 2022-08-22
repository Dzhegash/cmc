use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub chat: Vec<Value>,
    pub twitter: Vec<Value>,
    pub blog: Vec<Value>,
    pub fee: Vec<Value>,
    pub website: Vec<Value>,
}