use std::path::PathBuf;

use color_eyre::eyre::Result;
use tracing::Level;

use self::{subcommand::Subcommand, verbosity::Verbosity};
use super::config::Config;
use crate::util::{fs, optional_param::OptionalParam, project_dirs::ProjectDirs};

pub(crate) mod subcommand;
pub(crate) mod verbosity;

#[derive(Debug, Clone, Default, clap::Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(flatten)]
    verbosity: Verbosity,

    /// Path to souko config file
    #[clap(long = "config", env = "SOUKO_CONFIG")]
    config_path: Option<PathBuf>,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

impl Args {
    pub fn verbosity(&self) -> Option<Level> {
        self.verbosity.verbosity()
    }

    pub(crate) fn config_path(&'_ self) -> OptionalParam<PathBuf> {
        OptionalParam::new("config", self.config_path.clone(), || {
            ProjectDirs::get().config_dir().join("config.toml")
        })
    }

    pub(crate) fn config(&self) -> Result<Config> {
        let config_path = self.config_path();
        let config = fs::load_toml(&config_path)?.unwrap_or_default();
        Ok(config)
    }

    pub(crate) fn subcommand(&self) -> Option<&Subcommand> {
        self.subcommand.as_ref()
    }
}
