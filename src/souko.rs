use color_eyre::eyre::{eyre, Result};

use crate::{application::service::Service, infrastructure, presentation::Presentation};

#[derive(Debug)]
pub struct Souko {}

impl Souko {
    pub fn command() -> clap::Command {
        Presentation::command()
    }

    pub fn main() -> Result<()> {
        let presentation = Presentation::from_env();

        color_eyre::install()?;

        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_writer(std::io::stderr)
            .try_init()
            .map_err(|e| eyre!(e))?;

        let repository = infrastructure::repository();
        let service = Service::new(&repository);
        presentation.main(&service)
    }
}
