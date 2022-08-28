use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    config::{Config, Root},
    tilde_path::TildePath,
};

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
    pub(crate) fn root_paths(&self, config: &Config) -> BTreeMap<String, Root> {
        if let Some(root_path) = &self.root_path {
            root_path
                .iter()
                .enumerate()
                .map(|(i, path)| (format!("arg{i}"), Root::new(TildePath::from_expanded(path))))
                .collect()
        } else {
            config.root_map().map().clone()
        }
    }

    pub(crate) fn json(&self) -> bool {
        self.json
    }
}
