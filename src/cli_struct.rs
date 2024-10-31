use crate::autohold;
use crate::build;
use crate::buildsets;
use crate::enums;
use crate::functions;
use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "sol", author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Generate autocomplete
    #[arg(long = "generate", value_enum)]
    pub generator: Option<Shell>,

    /// SubCommands
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Tenants
    #[arg(global = true, long)]
    pub tenant: Option<String>,

    /// Limit response from server
    #[arg(global = true, long, default_value_t = 10)]
    pub limit: usize,

    /// Output format
    #[arg(global = true, long, default_value_t = enums::output::Output::USER)]
    pub output: enums::output::Output,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// Buildsets
    BuildSets(buildsets::cli::BuildSets),

    /// AutoHolds
    AutoHold(autohold::cli::AutoHold),

    /// Build information
    Build(build::cli::Build),

    /// Functions
    Functions(functions::cli::Functions),
}
