use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Deserializer};

use crate::project_dirs::ProjectDirs;

#[derive(Debug, Clone)]
pub(crate) struct Root {
    path: PathBuf,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            path: default_path(),
        }
    }
}

impl Root {
    pub(crate) fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self { path }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Deserialize)]
struct Pair {
    name: String,
    path: Option<PathBuf>,
}

pub(super) fn default() -> HashMap<String, Root> {
    let mut map = HashMap::new();
    map.insert("default".to_owned(), Root::default());
    map
}

pub(super) fn deserialize<'de, D>(des: D) -> Result<HashMap<String, Root>, D::Error>
where
    D: Deserializer<'de>,
{
    let pairs: Vec<Pair> = Deserialize::deserialize(des)?;

    let mut map = default();
    for Pair { name, path } in pairs {
        let root = Root {
            path: path.unwrap_or_else(default_path),
        };
        map.insert(name, root);
    }
    Ok(map)
}

fn default_path() -> PathBuf {
    ProjectDirs::get().data_local_dir().join("root")
}
