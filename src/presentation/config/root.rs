use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};

use crate::{
    domain::model::root::Root,
    presentation::util::{
        optional_param::OptionalParam, project_dirs::ProjectDirs, tilde_path::TildePath,
    },
};

type RootMapRepr = Vec<RootRepr>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootRepr {
    name: String,
    path: Option<TildePath>,
}

#[derive(Debug, Clone)]
pub(super) struct RootMap {
    map: BTreeMap<String, RootConfig>,
}

const DEFAULT_ROOT_NAME: &str = "default";

impl Default for RootMap {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert(DEFAULT_ROOT_NAME.to_string(), RootConfig::default());
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
            let root = path.map(RootConfig::new).unwrap_or_default();
            map.map.insert(name, root);
        }
        Ok(map)
    }
}

impl RootMap {
    pub(super) fn default_root(&self) -> OptionalParam<Root> {
        let name = DEFAULT_ROOT_NAME;
        self.map
            .get(name)
            .expect("BUG: default root not found")
            .to_root(name.to_owned())
    }

    pub(super) fn roots(&self) -> BTreeMap<String, OptionalParam<Root>> {
        self.map
            .iter()
            .map(|(name, root)| (name.clone(), root.to_root(name.clone())))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct RootConfig {
    path: OptionalParam<TildePath>,
}

impl Default for RootConfig {
    fn default() -> Self {
        let path = ProjectDirs::get().data_local_dir().join("root");
        let path = TildePath::from_real_path(path);
        let path = OptionalParam::new_default("root", path);
        Self { path }
    }
}

impl RootConfig {
    fn new(path: impl Into<TildePath>) -> Self {
        let path = path.into();
        let path = OptionalParam::new_explicit("root", path);
        Self { path }
    }

    fn to_root(&self, name: String) -> OptionalParam<Root> {
        self.path.as_ref().map(|path| Root::new(name, path.into()))
    }
}
