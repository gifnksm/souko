use std::env;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use souko::App;
use tracing::Level;

fn main() -> Result<()> {
    let app = App::parse();

    if env::var_os("RUST_LOG").is_none() {
        match app.verbosity() {
            Some(Level::ERROR) => env::set_var("RUST_LOG", "error"),
            Some(Level::WARN) => env::set_var("RUST_LOG", "warn"),
            Some(Level::INFO) => env::set_var("RUST_LOG", "info"),
            Some(Level::DEBUG) => env::set_var("RUST_LOG", "debug"),
            Some(Level::TRACE) => env::set_var("RUST_LOG", "trace"),
            None => env::set_var("RUST_LOG", "off"),
        }
    }

    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| eyre!(e))?;

    souko::main(&app)
}
