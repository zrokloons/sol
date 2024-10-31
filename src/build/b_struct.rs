use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
    project: String,
    branch: String,
    change: usize,
    patchset: String,
    #[serde(rename = "ref")]
    _ref: String,
    oldrev: Option<String>,
    newrev: Option<String>,
    ref_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSet {
    pub uuid: String,
    refs: Vec<Ref>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildResult {
    pub _id: usize,
    pub uuid: String,
    pub job_name: String,
    pub result: Option<String>,
    pub held: bool,
    pub start_time: String,
    pub end_time: String,
    pub duration: f64,
    pub voting: bool,
    pub log_url: String,
    pub nodeset: String,
    pub error_detail: Option<String>,
    #[serde(rename = "final")]
    pub _final: bool,
    pub artifacts: Vec<Artifact>,
    pub provides: Vec<String>,
    #[serde(rename = "ref")]
    pub _ref: Ref,
    pub pipeline: String,
    pub event_id: String,
    pub event_timestamp: String,
    pub buildset: BuildSet,
}
