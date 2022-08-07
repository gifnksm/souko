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

mod query;
mod repo;
mod repo_index;
mod template;
mod walk_repo;

pub use self::{query::*, repo::*, repo_index::*, template::*, walk_repo::*};
