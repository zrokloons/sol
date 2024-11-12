use anyhow::Result as AnyhowResult;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
use sol::autohold;
use sol::builds;
use sol::buildsets;
use sol::cli_struct::{Cli, Commands};
use sol::config::Config;
use sol::functions;
use std::io;

/*
 * Function that print to stdout the autocompletion file for
 * the shell selected
 */
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() -> AnyhowResult<()> {
    env_logger::init();
    let cli: Cli = Cli::parse();

    // generator
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
        return Ok(());
    }

    // Load configuration
    let config = Config::load(&cli)?;

    // Handle all commands
    match &cli.command {
        Some(Commands::Functions(func)) => match &func.command {
            functions::cli::FunctionCommand::BuildNodes(bn) => {
                functions::build_node::command::BuildNode::new(config)?
                    .build_id(bn.build_id.clone())?
                    .force(bn.force)?
                    .runner()?
                    .show()?;
            }
        },
        Some(Commands::Builds(builds)) => {
            builds::command::Builds::new(config)?
                .job_name(builds.job_name.clone())?
                .uuid(builds.uuid.clone())?
                .change(builds.change.clone())?
                .patchset(builds.patchset.clone())?
                .force(builds.force)?
                .verbose(builds.verbose)?
                .runner()?
                .show()?;
        }
        Some(Commands::BuildSets(bs)) => {
            buildsets::command::BuildSets::new(config)?
                .result(bs.result.clone())?
                .project(bs.group.project.clone())?
                .change(bs.group.change.clone())?
                .uuid(bs.group.uuid.clone())?
                .runner()?
                .show()?;
        }
        Some(Commands::AutoHold(ah)) => match &ah.command {
            autohold::cli::AutoHoldCommand::List { user, snapped } => {
                autohold::list_command::ListAutoHold::new(config)?
                    .user(user.clone())?
                    .snapped(*snapped)?
                    .runner()?
                    .filter()?
                    .show()?;
            }
        },
        None => {}
    }

    Ok(())
}
