use color_eyre::eyre::Result;

use crate::cli::{self, Args};

mod clone;
mod import;
mod list;

pub(crate) fn run(args: &Args) -> Result<()> {
    match args.subcommand() {
        cli::SubCommand::Clone(clone_args) => clone::run(args, clone_args)?,
        cli::SubCommand::Import(import_args) => import::run(args, import_args)?,
        cli::SubCommand::List => list::run(args)?,
    }
    Ok(())
}
