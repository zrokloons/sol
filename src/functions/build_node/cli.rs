use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
pub struct BuildNode {
    /// Build ID
    #[arg(required = true)]
    pub build_id: String,
}
