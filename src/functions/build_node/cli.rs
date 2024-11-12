use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
pub struct BuildNode {
    /// Build ID
    #[arg(required = true)]
    pub build_id: String,

    /// Don't use cache if any
    #[arg(long, short, default_value_t = false)]
    pub force: bool,
}
