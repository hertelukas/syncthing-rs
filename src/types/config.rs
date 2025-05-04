//! Config types, as defined [here](https://docs.syncthing.net/users/config.html)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub version: u64,
    pub folders: Vec<FolderConfiguration>,
    pub devices: Vec<DeviceConfiguration>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderConfiguration {
    pub id: String,
    pub label: String,
    pub path: String,
    pub devices: Vec<FolderDeviceConfiguration>,
    pub xattr_filter: XattrFilter,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct XattrFilter {
    pub entries: Vec<String>,
    pub max_single_entry_size: u64,
    pub max_total_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderDeviceConfiguration {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub introduced_by: String,
    pub encryption_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfiguration {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub name: String,
    pub addresses: Vec<String>, // TODO parse as SocketAddr or "dynamic"
    pub compression: Compression,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Compression {
    Metadata,
    Always,
    Never,
}
