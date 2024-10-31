use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
pub struct AutoHold {
    #[command(subcommand)]
    pub command: AutoHoldCommand,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum AutoHoldCommand {
    /// List buildsets
    List {
        /// Filter on requester
        #[arg(long, short)]
        user: Option<String>,

        /// Filter on snapped
        #[arg(long, short, default_value_t = false)]
        snapped: bool,
    },
}
