//! Config types, as defined [here](https://docs.syncthing.net/users/config.html)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub value: f64,
    pub unit: String,
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
    pub config_type: String,
    pub params: HashMap<String, String>,
    pub cleanup_interval_s: i64,
    pub fs_path: String,
    pub fs_type: FilesystemType,
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
    pub time: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GuiConfiguration {
    pub enabled: bool,
    pub address: String,
    pub unix_socket_permissions: String,
    pub user: String,
    pub password: String,
    pub auth_mode: AuthMode,
    // pub metrics_without_auth: bool,
    #[serde(rename = "useTLS")]
    pub use_tls: bool,
    pub api_key: String,
    pub insecure_admin_access: bool,
    pub theme: String,
    pub debugging: bool,
    pub insecure_skip_hostcheck: bool,
    pub insecure_allow_frame_loading: bool,
    pub send_basic_auth_prompt: bool,
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
pub struct LDAPConfiguration {
    pub address: String,
    #[serde(rename = "bindDN")]
    pub bind_dn: String,
    pub transport: LDAPTransport,
    pub insecure_skip_verify: bool,
    #[serde(rename = "searchBaseDN")]
    pub search_base_dn: String,
    pub search_filter: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LDAPTransport {
    Plain,
    NonTLS,
    TLS,
    StartTLS,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsConfiguration {
    pub listen_address: Vec<String>,
    pub global_announce_servers: Vec<String>,
    pub global_annouce_enabled: bool,
    pub local_announce_enabled: bool,
    pub local_announce_port: i64,
    #[serde(rename = "localAnnounceMCAddr")]
    pub local_announce_mc_addr: String,
    pub max_send_kbps: i64,
    pub max_recv_kbps: i64,
    pub reconnection_interval_s: i64,
    pub relays_enabled: bool,
    pub relay_reconnect_interval_m: i64,
    pub start_browser: bool,
    pub nat_enabled: bool,
    pub nat_lease_minutes: i64,
    pub nat_renewal_minutes: i64,
    pub nat_timeout_seconds: i64,
    pub ur_accepted: i64,
    pub ur_screen: i64,
    pub ur_unique_id: String,
    #[serde(rename = "urURL")]
    pub ur_url: String,
    pub ur_post_insecurely: bool,
    pub ur_initial_dleay_s: i64,
    pub auto_upgraade_interval_h: i64,
    pub upgrade_to_pre_releases: bool,
    pub keep_temporaries_h: i64,
    pub cache_ignored_files: bool,
    pub progress_update_inteval_s: i64,
    pub limit_bandwidth_in_lan: bool,
    pub min_home_disk_free: Size,
    #[serde(rename = "releasesURL")]
    pub releases_url: String,
    pub always_local_nets: Vec<String>,
    pub overwrite_remote_device_names_on_connect: bool,
    pub temp_index_min_blocks: i64,
    #[serde(rename = "unackedNotificationIDs")]
    pub unacked_notifications_ids: Vec<String>,
    pub traffic_class: i64,
    pub set_low_priority: bool,
    pub max_folder_concurrency: i64,
    #[serde(rename = "crURL")]
    pub cr_url: String,
    pub crash_reporting_enabled: bool,
    pub stun_keepalive_start_s: i64,
    pub stun_keepalive_min_s: i64,
    pub stun_servers: Vec<String>,
    pub database_tuning: Tuning,
    pub max_concurrent_incoming_requests_ki_b: i64,
    #[serde(rename = "announceLANAddresses")]
    pub announce_lan_addresses: bool,
    pub send_full_index_on_upgrade: bool,
    pub feature_flags: Vec<String>,
    pub audit_enabled: bool,
    pub audit_file: String,
    pub connection_limit_enough: i64,
    pub connection_limit_max: i64,
    pub connection_priority_tcp_lan: i64,
    pub connection_priority_quic_lan: i64,
    pub connection_priority_tcp_wan: i64,
    pub connection_priority_quic_wan: i64,
    pub connection_priority_relay: i64,
    pub connection_priority_upgrade_threshold: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Tuning {
    Small,
    Large,
    Auto,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObservedDevice {
    pub time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub name: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    pub folder: FolderConfiguration,
    pub device: DeviceConfiguration,
    pub ignores: Ignores,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ignores {
    lines: Vec<String>,
}
