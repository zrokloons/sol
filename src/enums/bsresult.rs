use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, clap::ValueEnum, Clone, PartialEq)]
pub enum BSResults {
    None,
    NoJobs,
    Success,
    Failure,
}

impl fmt::Display for BSResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            Self::None => "NONE",
            Self::NoJobs => "NO_JOBS",
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
        };
        write!(f, "{}", x)
    }
}

impl BSResults {
    pub fn as_str(&self) -> &'static str {
        match *self {
            BSResults::None => "NONE",
            BSResults::NoJobs => "NO_JOBS",
            BSResults::Success => "SUCCESS",
            BSResults::Failure => "FAILURE",
        }
    }
}
