use color_eyre::eyre::Result;

use crate::{presentation::args::subcommand::Subcommand, Args as AppArgs};

pub(crate) mod clone;
pub(crate) mod list;

pub(crate) fn run(app: &AppArgs) -> Result<()> {
    match app.subcommand() {
        Some(Subcommand::Clone(args)) => clone::run(app, args)?,
        Some(Subcommand::List(args)) => list::run(app, args)?,
        None => <AppArgs as clap::CommandFactory>::command().print_help()?,
    }
    Ok(())
}
