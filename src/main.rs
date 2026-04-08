use std::env;

use clap::Parser as _;
use color_eyre::eyre::{self, eyre};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{app_dirs::AppDirs, application::usecase::Usecases, presentation::args::Args};

#[macro_use]
mod macros;

mod app_dirs;
mod application;
mod domain;
mod infrastructure;
mod presentation;
mod util;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let env_prefix = BIN_NAME.to_uppercase().replace("-", "_");
    if let Ok(shell) = env::var(format!("{env_prefix}_COMPLETE")) {
        presentation::print_completion(BIN_NAME, &shell)?;
        return Ok(());
    }
    if let Ok(output_dir) = env::var(format!("{env_prefix}_GENERATE_MAN_TO")) {
        presentation::generate_man(&output_dir)?;
        return Ok(());
    }

    let args = Args::parse();
    let env_filter = if env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        let default_directive = args
            .global_args()
            .verbosity()
            .map(Into::into)
            .unwrap_or(LevelFilter::OFF.into());
        EnvFilter::builder()
            .with_default_directive(default_directive)
            .from_env_lossy()
    };

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| eyre!(e))?;

    let ports = infrastructure::ports();
    let usecases = Usecases::new(&ports);
    let app_dirs = AppDirs::new(BIN_NAME)?;
    presentation::dispatch(&args, usecases, app_dirs)
}
