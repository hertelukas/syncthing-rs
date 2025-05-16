//! All types required for the system endpoints
use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connections {
    pub connections: HashMap<String, Connection>,
    pub total: TotalConnections,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub address: String,
    pub at: chrono::DateTime<Utc>,
    pub client_version: String,
    pub connected: bool,
    pub in_bytes_total: i64,
    pub is_local: bool,
    pub out_bytes_total: i64,
    pub paused: bool,
    pub started_at: chrono::DateTime<Utc>,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalConnections {
    pub at: chrono::DateTime<Utc>,
    pub in_bytes_total: i64,
    pub out_bytes_total: i64,
}
