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

use color_eyre::eyre::Result;

pub use self::presentation::args::Args;
use self::util::project_dirs::ProjectDirs;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod util;

pub fn main(app: &Args) -> Result<()> {
    ProjectDirs::init()?;
    application::command::run(app)?;
    Ok(())
}
