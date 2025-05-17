//! All types required for the db endpoints
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    completion: f64,
    global_bytes: i64,
    need_bytes: i64,
    global_items: i64,
    need_items: i64,
    need_deletes: i64,
    remote_state: String,
    sequence: i64,
}
