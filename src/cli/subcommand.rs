pub(crate) mod clone;
pub(crate) mod import;
pub(crate) mod list;

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum Subcommand {
    /// Clone remote repositories and put them into souko
    Clone(clone::Args),
    /// Import local repositories into souko
    Import(import::Args),
    /// List repositories in souko
    List(list::Args),
}
