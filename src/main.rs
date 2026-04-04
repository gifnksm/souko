use std::env;

use color_eyre::eyre::{self, eyre};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{application::service::Service, presentation::Presentation, project_dirs::ProjectDirs};

#[macro_use]
mod macros;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod project_dirs;
mod util;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let env_prefix = BIN_NAME.to_uppercase().replace("-", "_");
    if let Ok(shell) = env::var(format!("{env_prefix}_COMPLETE")) {
        Presentation::print_completion(BIN_NAME, &shell)?;
        return Ok(());
    }
    if let Ok(output_dir) = env::var(format!("{env_prefix}_GENERATE_MAN_TO")) {
        Presentation::generate_man(&output_dir)?;
        return Ok(());
    }

    let presentation = Presentation::from_args(env::args_os());
    let env_filter = if env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        let default_directive = presentation
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
    let service = Service::new(&ports);
    let project_dirs = ProjectDirs::new()?;
    presentation.main(&service, &project_dirs)
}
