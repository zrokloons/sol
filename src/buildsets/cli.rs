use crate::enums::bsresult::BSResults;
use clap::Parser;
use serde_json::Number;

#[derive(Debug, Parser, PartialEq)]
pub struct BuildSets {
    /// Filter on result
    #[arg(long, short)]
    pub result: Option<BSResults>,

    /// Filter on project
    #[arg(long, short)]
    pub project: Option<String>,

    /// Filter on change
    #[arg(long, short)]
    pub change: Option<Number>,
}
