use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use directories::ProjectDirs;
use once_cell::sync::OnceCell;

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
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or_else(|| eyre!("failed to get project directories"))?;
    PROJECT_DIRS
        .set(project_dirs)
        .expect("BUG: faield to set project directories");

    let args = cli::Args::parse();
    command::run(&args)?;

    Ok(())
}
