use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSetRefs {
    pub project: String,
    pub branch: String,
    pub change: Value,
    pub patchset: Option<String>,
    #[serde(rename = "ref")]
    pub refff: String,
    pub oldrev: Option<String>,
    pub newref: Option<String>,
    pub ref_url: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSetsResult {
    pub _id: Value,
    pub uuid: String,
    pub result: Option<String>,
    pub message: Option<String>,
    pub pipeline: String,
    pub event_id: String,
    pub event_timestamp: Option<String>,
    pub first_build_start_time: Option<String>,
    pub last_build_end_time: Option<String>,
    pub refs: Vec<BuildSetRefs>,
}
