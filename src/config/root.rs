use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};

use crate::{optional_param::OptionalParam, project_dirs::ProjectDirs, tilde_path::TildePath};

type RootMapRepr = Vec<RootRepr>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootRepr {
    name: String,
    path: Option<TildePath>,
}

#[derive(Debug, Clone)]
pub(crate) struct RootMap {
    map: BTreeMap<String, Root>,
}

const DEFAULT_ROOT_NAME: &str = "default";

impl Default for RootMap {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert(DEFAULT_ROOT_NAME.to_string(), Root::default());
        Self { map }
    }
}

impl<'de> Deserialize<'de> for RootMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs: RootMapRepr = Deserialize::deserialize(deserializer)?;

        let mut map = Self::default();
        for RootRepr { name, path } in pairs {
            let root = path.map(Root::new).unwrap_or_default();
            map.map.insert(name, root);
        }
        Ok(map)
    }
}

impl RootMap {
    pub(crate) fn default_root(&self) -> &Root {
        self.map
            .get(DEFAULT_ROOT_NAME)
            .expect("BUG: default root not found")
    }

    pub(crate) fn map(&self) -> &BTreeMap<String, Root> {
        &self.map
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Root {
    path: OptionalParam<TildePath>,
}

impl Default for Root {
    fn default() -> Self {
        let path = ProjectDirs::get().data_local_dir().join("root");
        let path = OptionalParam::new_default("root", TildePath::from_expanded(path));
        Self { path }
    }
}

impl Root {
    pub(crate) fn new(path: impl Into<TildePath>) -> Self {
        let path = path.into();
        let path = OptionalParam::new_explicit("root", path);
        Self { path }
    }

    pub(crate) fn path(&self) -> &OptionalParam<TildePath> {
        &self.path
    }
}
