use crate::enums::bsresult::BSResults;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub project: Option<String>,
    pub result: Option<BSResults>,
    pub change: Option<Number>,
    pub uuid: Option<String>,
}
