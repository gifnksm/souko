use color_eyre::eyre::Result;

use super::GlobalArgs;
use crate::application::service::Service;

pub(crate) mod clone;
pub(crate) mod list;

#[derive(Debug, Clone, clap::Subcommand)]
pub(super) enum Subcommand {
    /// Clone remote repositories and put them into souko
    Clone(CloneArgs),
    /// List repositories in souko
    List(ListArgs),
}

impl Subcommand {
    pub(super) fn run(&self, global_args: &GlobalArgs, service: &Service) -> Result<()> {
        match self {
            Self::Clone(args) => args.inner.run(global_args),
            Self::List(args) => args.inner.run(global_args, service),
        }
    }
}

// To prevent leak of clone::Args and list::Args, we wrap them with our own Args
#[derive(Debug, Clone, clap::Args)]
pub(super) struct CloneArgs {
    #[clap(flatten)]
    inner: clone::Args,
}
#[derive(Debug, Clone, clap::Args)]
pub(super) struct ListArgs {
    #[clap(flatten)]
    inner: list::Args,
}
