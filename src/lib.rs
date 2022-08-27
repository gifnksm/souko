//! A simple command line utility that provides an easy way to organize clones of remote git repositories
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! souko = "0.0.0"
//! ```

#![doc(html_root_url = "https://docs.rs/souko/0.0.0")]

use color_eyre::eyre::Result;

mod cli;
mod command;
mod config;
mod optional_param;
mod project_dirs;
mod query;
mod repo;
mod repo_index;
mod scheme;
mod template;
mod walk_repo;

pub use self::cli::app::App;
use self::{
    config::Config, optional_param::OptionalParam, project_dirs::ProjectDirs, query::Query,
    repo::Repo, repo_index::RepoIndex, scheme::Scheme, template::Template, walk_repo::WalkRepo,
};

pub fn main(app: &App) -> Result<()> {
    ProjectDirs::init()?;
    command::run(app)?;
    Ok(())
}
