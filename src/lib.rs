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

use std::env;

use clap::Parser;
pub use color_eyre::eyre::Result;
use tracing::Level;

mod args;
mod command;
mod config;
mod optional_param;
mod project_dirs;
mod query;
mod repo;
mod repo_index;
mod template;
mod walk_repo;

use self::{
    args::Args, config::Config, optional_param::OptionalParam, project_dirs::ProjectDirs,
    query::Query, repo::Repo, repo_index::RepoIndex, template::Template, walk_repo::*,
};

pub fn main() -> Result<()> {
    let args = Args::parse();
    if env::var_os("RUST_LOG").is_none() {
        match args.verbosity() {
            Some(Level::ERROR) => env::set_var("RUST_LOG", "error"),
            Some(Level::WARN) => env::set_var("RUST_LOG", "warn"),
            Some(Level::INFO) => env::set_var("RUST_LOG", "info"),
            Some(Level::DEBUG) => env::set_var("RUST_LOG", "debug"),
            Some(Level::TRACE) => env::set_var("RUST_LOG", "trace"),
            None => env::set_var("RUST_LOG", "off"),
        }
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    color_eyre::install()?;

    ProjectDirs::init()?;

    command::run(&args)?;

    Ok(())
}
