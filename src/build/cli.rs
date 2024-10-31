use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
pub struct Build {
    /// Build ID
    pub build_id: String,

    /// Force request to API (rewrite cache)
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}
