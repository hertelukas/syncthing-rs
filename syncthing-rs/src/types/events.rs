//! Event types, as defined [here](https://docs.syncthing.net/dev/events.html)
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::config::{
    Defaults, DeviceConfiguration, FolderConfiguration, GuiConfiguration, LDAPConfiguration,
    NewDeviceConfiguration, ObservedDevice,
};

/// Represents an [Event](https://docs.syncthing.net/dev/events.html)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: u64,
    #[serde(rename = "globalID")]
    pub global_id: u64,
    pub time: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub ty: EventType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    ClusterConfigReceived {
        device: String,
    },
    #[serde(rename_all = "camelCase")]
    ConfigSaved {
        version: u64,
        folders: Vec<FolderConfiguration>,
        devices: Vec<DeviceConfiguration>,
        gui: GuiConfiguration,
        ldap: LDAPConfiguration,
        remote_ignored_devices: Vec<ObservedDevice>,
        defaults: Box<Defaults>,
    },
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
    DeviceDiscovered {
        addrs: Vec<String>,
        device: String,
    },
    DevicePaused {
        device: String,
    },
    DeviceRejected {}, // Deprecated
    DeviceResumed {
        device: String,
    },
    DownloadProgress {
        #[serde(flatten)]
        folders: HashMap<String, HashMap<String, FileDownloadProgress>>,
    },
    Failure(String),
    #[serde(rename_all = "camelCase")]
    FolderCompletion {
        completion: f64,
        device: String,
        folder: String,
        global_bytes: u64,
        global_items: u64,
        need_bytes: u64,
        need_deletes: u64,
        need_items: u64,
        remote_state: String,
        sequence: u64,
    },
    FolderErrors {
        errors: Vec<FolderError>,
        folder: String,
    },
    FolderPaused {
        id: String,
        label: String,
    },
    FolderRejected {}, // Deprecated
    FolderResumed {
        id: String,
        label: String,
    },
    FolderScanProgress {
        total: u64,
        rate: u64,
        current: u64,
        folder: String,
    },
    FolderSummary {
        folder: String,
        summary: FolderSummary,
    },
    FolderWatchStateChanged {
        folder: String,
        from: String,
        to: String,
    },
    ItemFinished {
        item: String,
        folder: String,
        error: Option<String>,
        #[serde(rename = "type")]
        ty: String,
        action: String,
    },
    ItemStarted {
        item: String,
        folder: String,
        #[serde(rename = "type")]
        ty: String,
        action: String,
    },
    ListenAddressesChanged {
        address: ListenAddressChanged,
        wan: Option<Vec<ListenAddressChanged>>,
        lan: Option<Vec<ListenAddressChanged>>,
    },
    LocalChangeDetected {
        action: String,
        folder: String,
        #[serde(rename = "folderID")]
        folder_id: String,
        label: String,
        path: String,
        #[serde(rename = "type")]
        ty: String,
    },
    LocalIndexUpdated {
        folder: String,
        items: u64,
        filenames: Vec<String>,
        sequence: u64,
    },
    #[serde(rename_all = "camelCase")]
    LoginAttempt {
        remote_address: String,
        username: String,
        success: bool,
        proxy: Option<String>,
    },
    PendingDevicesChanged {
        added: Option<Vec<AddedPendingDeviceChanged>>,
        removed: Option<Vec<RemovedPendingDeviceChanged>>,
    },
    PendingFoldersChanged {
        added: Option<Vec<AddedPendingFolderChanged>>,
        removed: Option<Vec<RemovedPendingFolderChanged>>,
    },
    #[serde(rename_all = "camelCase")]
    RemoteChangeDetected {
        #[serde(rename = "type")]
        ty: String,
        action: String,
        folder: String,
        #[serde(rename = "folderID")]
        folder_id: String,
        path: String,
        label: String,
        modified_by: String,
    },
    RemoteDownloadProgress {
        state: HashMap<String, u64>,
        device: String,
        folder: String,
    },
    RemoteIndexUpdated {
        device: String,
        folder: String,
        items: u64,
    },
    Starting {
        home: String,
    },
    StartupComplete {
        #[serde(rename = "myID")]
        my_id: String,
    },
    StateChanged {
        folder: String,
        from: String,
        duration: Option<f64>,
        to: String,
    },
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FolderError {
    pub error: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FolderSummary {
    pub errors: i64,
    pub global_files: i64,
    pub global_directories: i64,
    pub global_symlinks: i64,
    pub global_deleted: i64,
    pub global_bytes: i64,
    pub global_total_items: i64,
    pub local_files: i64,
    pub local_directories: i64,
    pub local_symlinks: i64,
    pub local_deleted: i64,
    pub local_bytes: i64,
    pub local_total_items: i64,
    pub need_files: i64,
    pub need_directories: i64,
    pub need_symlinks: i64,
    pub need_deletes: i64,
    pub need_bytes: i64,
    pub need_total_items: i64,
    pub receive_only_changed_files: i64,
    pub receive_only_changed_directories: i64,
    pub receive_only_changed_symlinks: i64,
    pub receive_only_changed_deletes: i64,
    pub receive_only_changed_bytes: i64,
    pub receive_only_total_items: i64,
    pub in_sync_files: i64,
    pub in_sync_bytes: i64,
    pub state: String,
    pub state_changed: chrono::DateTime<chrono::Utc>,
    pub error: String,
    pub sequence: i64,
    pub remote_sequence: HashMap<String, i64>,
    pub ignore_patterns: bool,
    pub watch_error: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListenAddressChanged {
    pub fragment: String,
    pub raw_query: String,
    pub scheme: String,
    pub path: String,
    pub user: Option<String>,
    pub force_query: bool,
    pub host: String,
    pub opaque: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileDownloadProgress {
    pub total: u64,
    pub pulling: u64,
    pub copied_from_original: u64,
    pub reused: u64,
    pub copied_from_elsewhere: u64,
    pub pulled: u64,
    pub bytes_total: u64,
    pub bytes_done: u64,
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
