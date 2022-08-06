use std::path::PathBuf;

use clap::Parser;
use tracing::Level;

use crate::{project_dirs, util::OptionalParam};

mod clone;
mod import;

pub(crate) use self::{clone::*, import::*};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {
    /// More output per occurrence
    #[clap(long, short = 'v', parse(from_occurrences), global = true)]
    verbose: i8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        parse(from_occurrences),
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: i8,
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
    pub(crate) fn verbosity(&self) -> Option<Level> {
        let level = self.verbose - self.quiet;
        match level {
            i8::MIN..=-3 => None,
            -2 => Some(Level::ERROR),
            -1 => Some(Level::WARN),
            0 => Some(Level::INFO),
            1 => Some(Level::DEBUG),
            2..=i8::MAX => Some(Level::TRACE),
        }
    }

    pub(crate) fn config(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("config", &self.config, || {
            project_dirs::get().config_dir().join("config.toml")
        })
    }

    pub(crate) fn repo_index(&'_ self) -> OptionalParam<'_, PathBuf> {
        OptionalParam::new("repository index", &self.repo_index, || {
            project_dirs::get().data_local_dir().join("repo_index.json")
        })
    }

    pub(crate) fn subcommand(&self) -> &SubCommand {
        &self.subcommand
    }
}
