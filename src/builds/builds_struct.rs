use crate::util::helpers;
use anyhow::Result as AnyhowResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifacts {
    name: String,
    url: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
    pub project: String,
    pub branch: String,
    pub change: usize,
    pub patchset: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub oldrev: Option<String>,
    pub newrev: Option<String>,
    pub ref_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSet {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildsResult {
    pub _id: usize,
    pub artifacts: Vec<Artifacts>,
    pub buildset: BuildSet,
    pub duration: Option<f64>,
    pub end_time: Option<String>,
    pub event_id: String,
    pub event_timestamp: String,
    #[serde(rename = "final")]
    pub _final: Option<bool>,
    pub held: Option<bool>,
    pub job_name: String,
    pub log_url: Option<String>,
    pub nodeset: String,
    pub pipeline: String,
    pub provides: Vec<String>,
    pub uuid: String,
    pub result: Option<String>,
    pub start_time: String,
    pub voting: bool,
    pub error_detail: Option<String>,
    #[serde(rename = "ref")]
    pub _ref: Ref,
    #[serde(flatten)]
    pub other: serde_json::Value,
}

#[derive(Debug)]
pub struct Target {
    pub dir: PathBuf,
    pub uuid: PathBuf,
    pub inventory: PathBuf,
    pub change: Option<String>,
    pub job_name: Option<String>,
}

impl Target {
    pub fn new(uuid: String, base_path: String) -> Target {
        let dir = format!("{base_path}/{uuid}");
        let inventory = "inventory.yaml.gz".to_string();
        Target {
            dir: Path::new(&dir).to_owned(),
            uuid: Path::new(&format!("{dir}/{uuid}")).to_owned(),
            inventory: Path::new(&format!("{dir}/{inventory}")).to_owned(),
            change: None,
            job_name: None,
        }
    }

    pub fn delete(&self) -> AnyhowResult<()> {
        log::debug!("Remove build directory: {}", self.dir.to_str().unwrap());
        if self.dir.exists() {
            helpers::remove_dir_files(&self.dir)?;
            std::fs::remove_dir(&self.dir)?;
        }
        Ok(())
    }
}
