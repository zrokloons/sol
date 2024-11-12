use clap::Args;

#[derive(Debug, Args, PartialEq, Clone)]
#[group(required = true)]
pub struct Builds {
    /// Ask for job name
    #[arg(
        long,
        short,
        conflicts_with = "uuid",
        conflicts_with = "change",
        conflicts_with = "patchset"
    )]
    pub job_name: Option<String>,

    /// Ask for uuid
    #[arg(
        long,
        short,
        conflicts_with = "job_name",
        conflicts_with = "change",
        conflicts_with = "patchset"
    )]
    pub uuid: Option<String>,

    /// Ask for change
    #[arg(long, short, group = "_change")]
    pub change: Option<String>,

    /// Ask for Patchset requires --change option
    #[arg(long, short, requires("_change"))]
    pub patchset: Option<String>,

    /// Force a new request
    #[arg(long, short, default_value_t = false)]
    pub force: bool,

    /// Verbose output (user)
    #[arg(long, short, default_value_t = false)]
    pub verbose: bool,
}
