use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Message {
    pub payload: String,
    pub delivery_tag: u64,
    pub ack: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SyncRequest {
    Write {
        write: WriteData,
    },
    Delete {
        delete: DeleteData,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WriteData {
    pub key: String,
    #[serde(flatten)]
    pub value: Value,
    pub resource: String,
    pub params: Option<HashMap<String, String>>,
    pub store: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteData {
    pub key: String,
    pub value: DeleteValue,
    pub resource: String,
    pub params: Option<HashMap<String, String>>,
    pub store: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteValue {
    pub _timestamp: Option<f64>,
}