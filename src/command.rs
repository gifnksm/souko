use color_eyre::eyre::Result;

use crate::{args::SubCommand, Args};

mod clone;
mod import;
mod list;

pub(crate) fn run(args: &Args) -> Result<()> {
    match args.subcommand() {
        SubCommand::Clone(clone_args) => clone::run(args, clone_args)?,
        SubCommand::Import(import_args) => import::run(args, import_args)?,
        SubCommand::List(list_args) => list::run(args, list_args)?,
    }
    Ok(())
}
