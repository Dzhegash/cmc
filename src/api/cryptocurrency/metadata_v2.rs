use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcMetadata {
    pub status: Status,
    pub data: HashMap<String, Metadata>,
}

