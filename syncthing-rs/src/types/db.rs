//! All types required for the db endpoints
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub completion: f64,
    pub global_bytes: i64,
    pub need_bytes: i64,
    pub global_items: i64,
    pub need_items: i64,
    pub need_deletes: i64,
    pub remote_state: String,
    pub sequence: i64,
}
