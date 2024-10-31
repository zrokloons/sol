use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Serialize, Deserialize, Clone, ValueEnum, Debug, Subcommand, PartialEq, Copy, Eq, Hash,
)]
pub enum Output {
    JSON,
    USER,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            Self::JSON => "json",
            Self::USER => "user",
        };
        write!(f, "{}", x)
    }
}
