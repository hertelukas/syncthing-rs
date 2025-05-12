//! Event types, as defined [here](https://docs.syncthing.net/dev/events.html)
use serde::{Deserialize, Serialize};

use super::config::NewDeviceConfiguration;

/// Represents an [Event](https://docs.syncthing.net/dev/events.html)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: u64,
    #[serde(rename = "globalID")]
    global_id: u64,
    time: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub ty: EventType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    ClusterConfigReceived {},
    ConfigSaved {},
    #[serde(rename_all = "camelCase")]
    DeviceConnected {
        addr: String,
        id: String,
        device_name: String,
        client_name: String,
        client_version: String,
        #[serde(rename = "type")]
        ty: ConnectionType,
    },
    DeviceDisconnected {
        error: String,
        id: String,
    },
    DeviceDiscovered {},
    DevicePause {},
    DeviceRejected {}, // Deprecated
    DeviceResumed {},
    DownloadProgress {},
    Failure {},
    FolderCompletion {},
    FolderErrors {},
    FolderPaused {},
    FolderRejected {}, // Deprecated
    FolderResumed {},
    FolderScanProgress {},
    FolderSummary {},
    FolderWatchStateChanged {},
    ItemFinished {},
    ItemStarted {},
    ListenAddressesChanged {},
    LocalChangeDetected {},
    LocalIndexUpdated {},
    LoginAttempt {},
    PendingDevicesChanged {
        added: Option<Vec<AddedPendingDeviceChanged>>,
        removed: Option<Vec<RemovedPendingDeviceChanged>>,
    },
    PendingFoldersChanged {
        added: Option<Vec<AddedPendingFolderChanged>>,
        removed: Option<Vec<RemovedPendingFolderChanged>>,
    },
    RemoteChangeDetected {},
    RemoteDownloadProgress {},
    RemoteIndexUpdated {},
    Starting {},
    StartupComplete {},
    StateChanged {},
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConnectionType {
    #[serde(rename = "tcp-client")]
    TCPClient,
    #[serde(rename = "tcp-server")]
    TCPServer,
    #[serde(rename = "relay-client")]
    RelayClient,
    #[serde(rename = "relay-server")]
    RelayServer,
    #[serde(rename = "quic-server")]
    QuicServer,
}

/// Information provided by the API if there is a new pending device
/// in a [`PendingDeviceChanged`](https://docs.syncthing.net/events/pendingdeviceschanged.html)
/// event.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AddedPendingDeviceChanged {
    pub address: std::net::SocketAddr,
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub name: String,
}

/// Information provided by the API if there is a pending device removed
/// in a [`PendingDeviceChanged`](https://docs.syncthing.net/events/pendingdeviceschanged.html)
/// event.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RemovedPendingDeviceChanged {
    #[serde(rename = "deviceID")]
    pub device_id: String,
}

/// Information provided by the API if there is a new pending folder
/// in a [`PendingFoldersChanged`](https://docs.syncthing.net/events/pendingfolderschanged.html)
/// event.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddedPendingFolderChanged {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    #[serde(rename = "folderID")]
    pub folder_id: String,
    pub folder_label: String,
    pub receive_encrypted: bool,
    pub remote_encrypted: bool,
}

/// Information provided by the API if there is a pending folder removed
/// in a [`PendingFoldersChanged`](https://docs.syncthing.net/events/pendingfolderschanged.html)
/// event.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct RemovedPendingFolderChanged {
    /// A removed entry without `device_id`, means that the folder is
    /// no longer pending on any device.
    #[serde(rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(rename = "folderID")]
    pub folder_id: String,
}

impl From<AddedPendingDeviceChanged> for NewDeviceConfiguration {
    fn from(value: AddedPendingDeviceChanged) -> Self {
        Self::new(value.device_id).name(value.name)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    #[test]
    fn test_new_device() {
        let added = AddedPendingDeviceChanged {
            address: std::net::SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8384),
            device_id: "foo".to_string(),
            name: "bar".to_string(),
        };

        let new: NewDeviceConfiguration = added.into();

        assert_eq!(new.get_device_id(), "foo");
        assert_eq!(new.get_name(), &Some("bar".to_string()));
    }
}
