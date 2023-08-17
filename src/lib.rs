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

pub use color_eyre::eyre::Result;

pub use crate::souko::Souko;

#[macro_use]
mod macros;

mod application;
mod domain;
mod infrastructure;
mod presentation;

mod souko {
    use color_eyre::eyre::Result;

    use crate::{application::service::Service, infrastructure, presentation::Presentation};

    #[derive(Debug)]
    pub struct Souko {
        presentation: Presentation,
    }

    impl Souko {
        pub fn from_env() -> Self {
            let presentation = Presentation::from_env();
            Self { presentation }
        }

        pub fn command() -> clap::Command {
            Presentation::command()
        }

        pub fn verbosity(&self) -> Option<tracing::Level> {
            self.presentation.verbosity()
        }

        pub fn main(self) -> Result<()> {
            let repository = infrastructure::repository();
            let service = Service::new(&repository);
            self.presentation.main(&service)
        }
    }
}
