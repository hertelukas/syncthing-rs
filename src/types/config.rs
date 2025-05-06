//! Config types, as defined [here](https://docs.syncthing.net/users/config.html)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    value: f64,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub version: u64,
    pub folders: Vec<FolderConfiguration>,
    pub devices: Vec<DeviceConfiguration>,
    pub gui: GuiConfiguration,
    pub ldap: LDAPConfiguration,
    pub remote_ignored_devices: Vec<ObservedDevice>,
    pub defaults: Defaults,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderConfiguration {
    pub id: String,
    pub label: String,
    pub filesystem_type: FilesystemType,
    pub path: String,
    #[serde(rename = "type")]
    pub folder_type: FolderType,
    pub devices: Vec<FolderDeviceConfiguration>,
    pub rescan_interval_s: i64,
    pub fs_watcher_enabled: bool,
    pub fs_watcher_delay_s: f64,
    pub fs_watcher_timeout_s: f64,
    pub ignore_perms: bool,
    pub auto_normalize: bool,
    pub min_disk_free: Size,
    pub versioning: VersioningConfiguration,
    pub copiers: i64,
    pub puller_max_pending_ki_b: i64,
    pub hashers: i64,
    pub order: PullOrder,
    pub ignore_delete: bool,
    pub scan_progress_interval_s: i64,
    pub puller_pause_s: i64,
    pub max_conflicts: i64,
    pub disable_sparse_files: bool,
    pub disable_temp_indexes: bool,
    pub paused: bool,
    pub weak_hash_threshold_pct: i64,
    pub marker_name: String,
    pub copy_ownership_from_parent: bool,
    pub mod_time_window_s: i64,
    pub max_concurrent_writes: i64,
    pub disable_fsync: bool,
    pub block_pull_order: BlockPullOrder,
    pub copy_range_method: CopyRangeMethod,
    #[serde(rename = "caseSensitiveFS")]
    pub case_sensitive_fs: bool,
    pub junctions_as_dirs: bool,
    pub sync_ownership: bool,
    pub send_ownership: bool,
    pub sync_xattrs: bool,
    pub send_xattrs: bool,
    pub xattr_filter: XattrFilter,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FilesystemType {
    Basic,
    Fake,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FolderType {
    SendReceive,
    SendOnly,
    ReceiveOnly,
    ReceiveEncrypted,
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
pub struct VersioningConfiguration {
    #[serde(rename = "type")]
    config_type: String,
    params: HashMap<String, String>,
    cleanup_interval_s: i64,
    fs_path: String,
    fs_type: FilesystemType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PullOrder {
    Random,
    Alphabetic,
    SmallestFirst,
    LargestFirst,
    OldestFirst,
    NewestFirst,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BlockPullOrder {
    Standard,
    Random,
    InOrder,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CopyRangeMethod {
    Standard,
    #[serde(rename = "copy_file_range")]
    CopyFileRange,
    Ioctl,
    SendFile,
    #[serde(rename = "duplicate_extents")]
    DuplicateExtents,
    All,
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
pub struct DeviceConfiguration {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub name: String,
    pub addresses: Vec<String>, // TODO parse as SocketAddr or "dynamic"
    pub compression: Compression,
    pub cert_name: String,
    pub introducer: bool,
    pub skip_introduction_removals: bool,
    pub introduced_by: String,
    pub paused: bool,
    pub allowed_networks: Vec<String>,
    pub auto_accept_folders: bool,
    pub max_send_kbps: i64,
    pub max_recv_kbps: i64,
    pub ignored_folders: Vec<ObservedFolder>,
    pub max_request_ki_b: i64,
    pub untrusted: bool,
    #[serde(rename = "remoteGUIPort")]
    pub remote_gui_port: i64,
    pub num_connections: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Compression {
    Metadata,
    Always,
    Never,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObservedFolder {
    time: chrono::DateTime<chrono::Utc>,
    id: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GuiConfiguration {
    enabled: bool,
    address: String,
    unix_socket_permissions: String,
    user: String,
    password: String,
    auth_mode: AuthMode,
    // metrics_without_auth: bool,
    #[serde(rename = "useTLS")]
    use_tls: bool,
    api_key: String,
    insecure_admin_access: bool,
    theme: String,
    debugging: bool,
    insecure_skip_hostcheck: bool,
    insecure_allow_frame_loading: bool,
    send_basic_auth_prompt: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthMode {
    #[serde(rename = "static")]
    StaticAuth,
    #[serde(rename = "ldap")]
    LDAP,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LDAPConfiguration {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsConfiguration {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObservedDevice {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {}
