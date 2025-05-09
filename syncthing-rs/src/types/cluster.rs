//! All types required for the cluster endpoints

use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingDevices {
    #[serde(flatten)]
    pub devices: HashMap<String, PendingDevice>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingDevice {
    pub time: chrono::DateTime<Utc>,
    pub name: String,
    pub address: std::net::SocketAddr,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingFolders {
    #[serde(flatten)]
    pub folders: HashMap<String, PendingFolder>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingFolder {
    /// Maps deviceID to the information about that folder on that device
    pub offered_by: HashMap<String, PendingFolderOfferer>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingFolderOfferer {
    time: chrono::DateTime<Utc>,
    pub label: String,
    receive_encrypted: bool,
    remote_encrypted: bool,
}
