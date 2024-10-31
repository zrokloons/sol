use crate::functions::build_node;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
pub struct Functions {
    #[command(subcommand)]
    pub command: FunctionCommand,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum FunctionCommand {
    /// Build Node information
    BuildNodes(build_node::cli::BuildNode),
}
