use crate::enums::bsresult::BSResults;
use clap::{Args, Parser};
use serde_json::Number;

#[derive(Debug, Parser, PartialEq)]
pub struct BuildSets {
    /// Filter on result
    #[arg(long, short)]
    pub result: Option<BSResults>,

    #[clap(flatten)]
    pub group: OptionsGroup,
}

#[derive(Debug, Args, PartialEq)]
#[group(required = true, multiple = false)]
pub struct OptionsGroup {
    /// Ask for project
    #[arg(long, short)]
    pub project: Option<String>,

    /// Ask for change
    #[arg(long, short)]
    pub change: Option<Number>,

    /// Ask for uuid
    #[arg(long, short)]
    pub uuid: Option<String>,
}
