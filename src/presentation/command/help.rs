use clap::CommandFactory;
use color_eyre::eyre::Result;

use crate::presentation::args::Args;

pub(super) fn dispatch() -> Result<()> {
    Args::command().print_help()?;
    Ok(())
}
