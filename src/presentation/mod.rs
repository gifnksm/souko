use clap::{CommandFactory as _, Parser as _};
pub use color_eyre::eyre::Result;

use self::{args::Args, util::project_dirs::ProjectDirs};
use crate::application::service::Service;

mod args;
mod config;
mod util;

#[derive(Debug)]
pub(crate) struct Presentation {
    args: Args,
}

impl Presentation {
    pub(crate) fn from_env() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub(crate) fn command() -> clap::Command {
        Args::command()
    }

    pub(crate) fn verbosity(&self) -> Option<tracing::Level> {
        self.args.verbosity()
    }

    pub(crate) fn main(self, service: &Service) -> Result<()> {
        ProjectDirs::init()?;
        self.args.run(service)?;
        Ok(())
    }
}
