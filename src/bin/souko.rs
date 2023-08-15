use std::env;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use souko::Args;

fn main() -> Result<()> {
    let app = Args::parse();

    if env::var_os("RUST_LOG").is_none() {
        let level_str = args
            .verbosity()
            .map(|level| level.as_str())
            .unwrap_or("off");
        env::set_var("RUST_LOG", level_str)
    }

    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| eyre!(e))?;

    souko::main(&app)
}
