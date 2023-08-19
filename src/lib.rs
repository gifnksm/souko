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

mod souko;
