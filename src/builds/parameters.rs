use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub job_name: Option<String>,
    pub change: Option<String>,
    pub patchset: Option<String>,
    pub uuid: Option<String>,
    pub force: bool,
    pub verbose: bool,
}
