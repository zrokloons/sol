use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub build_id: String,
    pub force: bool,
}
