use std::{collections::HashMap, path::PathBuf};

use crate::config::{Config, Root};

#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct Args {
    /// Path of the root directory under which the repository will be cloned
    #[clap(long = "root")]
    root_path: Option<Vec<PathBuf>>,

    /// Output as JSON
    #[clap(long)]
    json: bool,
}

impl Args {
    pub(crate) fn root_paths(&self, config: &Config) -> HashMap<String, Root> {
        if let Some(root_path) = &self.root_path {
            root_path
                .iter()
                .enumerate()
                .map(|(i, path)| (format!("arg{i}"), Root::new(path.to_owned())))
                .collect()
        } else {
            config.root_map().clone()
        }
    }

    pub(crate) fn json(&self) -> bool {
        self.json
    }
}
