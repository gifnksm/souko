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

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod util;

mod souko {
    use clap::{CommandFactory as _, Parser as _};
    use color_eyre::eyre::Result;

    use crate::{presentation::args::Args, util::project_dirs::ProjectDirs};

    #[derive(Debug)]
    pub struct Souko {
        args: Args,
    }

    impl Souko {
        #[allow(clippy::new_without_default)]
        pub fn from_env() -> Self {
            let args = Args::parse();
            Self { args }
        }

        pub fn command() -> clap::Command {
            Args::command()
        }

        pub fn verbosity(&self) -> Option<tracing::Level> {
            self.args.verbosity()
        }

        pub fn main(self) -> Result<()> {
            ProjectDirs::init()?;
            self.args.run()?;
            Ok(())
        }
    }
}
