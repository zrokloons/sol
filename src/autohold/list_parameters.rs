use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub user: Option<String>,
    pub snapped: bool,
}
