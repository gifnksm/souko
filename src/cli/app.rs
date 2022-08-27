use std::path::PathBuf;

use tracing::Level;

use crate::{optional_param::OptionalParam, project_dirs::ProjectDirs};

use super::{args::Verbosity, subcommand::Subcommand};

#[derive(Debug, Clone, Default, clap::Parser)]
#[clap(author, version, about)]
pub struct App {
    #[clap(flatten)]
    verbosity: Verbosity,

    /// Path to souko config file
    #[clap(long, env = "SOUKO_CONFIG")]
    config: Option<PathBuf>,
    /// Path to souko repository index
    #[clap(long, env = "SOUKO_REPO_INDEX")]
    repo_index: Option<PathBuf>,
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

impl App {
    pub fn verbosity(&self) -> Option<Level> {
        self.verbosity.verbosity()
    }

    pub(crate) fn config(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("config", &self.config, || {
            ProjectDirs::get().config_dir().join("config.toml")
        })
    }

    pub(crate) fn repo_index(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("repository index", &self.repo_index, || {
            ProjectDirs::get().data_local_dir().join("repo_index.json")
        })
    }

    pub(crate) fn subcommand(&self) -> Option<&Subcommand> {
        self.subcommand.as_ref()
    }
}
