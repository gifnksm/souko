use color_eyre::eyre::Result;

use crate::{cli::subcommand::Subcommand, App};

pub(crate) mod clone;
pub(crate) mod list;

pub(crate) fn run(app: &App) -> Result<()> {
    match app.subcommand() {
        Some(Subcommand::Clone(args)) => clone::run(app, args)?,
        Some(Subcommand::List(args)) => list::run(app, args)?,
        None => <App as clap::CommandFactory>::command().print_help()?,
    }
    Ok(())
}
