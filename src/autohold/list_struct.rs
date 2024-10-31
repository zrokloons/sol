use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    pub build: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoHoldResult {
    pub id: String,
    pub tenant: String,
    pub project: String,
    pub job: String,
    pub ref_filter: String,
    pub max_count: usize,
    pub current_count: usize,
    pub reason: String,
    pub node_expiration: usize,
    pub expired: Option<f64>,
    pub nodes: Vec<Nodes>,
}
