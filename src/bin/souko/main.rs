use std::env;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use tracing::Level;

mod cli;
mod command;
mod config;
mod util;

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

#[track_caller]
fn project_dirs() -> &'static ProjectDirs {
    PROJECT_DIRS
        .get()
        .expect("BUG: project_dirs() called before init()")
}

fn main() -> Result<()> {
    let args = cli::Args::parse();
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
        .with_writer(std::io::stderr)
        .init();
    color_eyre::install()?;

    let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or_else(|| eyre!("failed to get project directories"))?;
    PROJECT_DIRS
        .set(project_dirs)
        .expect("BUG: faield to set project directories");

    command::run(&args)?;

    Ok(())
}
