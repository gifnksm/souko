use std::env;

use clap::Parser;
use color_eyre::eyre::Result;
use tracing::Level;

mod cli;
mod command;
mod config;
mod project_dirs;
mod util;

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
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    color_eyre::install()?;

    project_dirs::init()?;

    command::run(&args)?;

    Ok(())
}
