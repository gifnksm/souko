use std::path::PathBuf;

use clap::Parser;

use crate::{project_dirs, util::OptionalParam};

mod clone;
mod import;

pub(crate) use self::{clone::*, import::*};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {
    /// Path to souko config file
    #[clap(long, env = "SOUKO_CONFIG")]
    config: Option<PathBuf>,
    /// Path to souko repository index
    #[clap(long, env = "SOUKO_REPO_INDEX")]
    repo_index: Option<PathBuf>,
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Parser)]
pub(crate) enum SubCommand {
    /// Clone remote repositories and put them into souko
    Clone(Clone),
    /// Import local repositories into souko
    Import(Import),
    /// List repositories in souko
    List,
}

impl Args {
    pub(crate) fn config(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("config", &self.config, || {
            project_dirs().config_dir().join("config.toml")
        })
    }

    pub(crate) fn repo_index(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("repository index", &self.repo_index, || {
            project_dirs().data_local_dir().join("repo_index.json")
        })
    }

    pub(crate) fn subcommand(&self) -> &SubCommand {
        &self.subcommand
    }
}
