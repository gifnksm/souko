use std::env;

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
        let args = Args::parse();

        if env::var_os("RUST_LOG").is_none() {
            let level_str = args
                .verbosity()
                .map(|level| level.as_str())
                .unwrap_or("off");
            env::set_var("RUST_LOG", level_str)
        }

        Self { args }
    }

    pub(crate) fn command() -> clap::Command {
        Args::command()
    }

    pub(crate) fn main(self, service: &Service) -> Result<()> {
        ProjectDirs::init()?;
        self.args.run(service)?;
        Ok(())
    }
}
