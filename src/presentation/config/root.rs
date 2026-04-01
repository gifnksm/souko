use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};

use crate::{
    domain::model::root::Root,
    presentation::model::{optional_param::OptionalParam, tilde_path::TildePath},
};

type RootMapRepr = Vec<RootRepr>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootRepr {
    name: String,
    path: Option<TildePath>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct RootMap {
    map: BTreeMap<String, RootConfig>,
}

const DEFAULT_ROOT_NAME: &str = "default";

impl<'de> Deserialize<'de> for RootMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs: RootMapRepr = Deserialize::deserialize(deserializer)?;

        let mut map = Self::default();
        for RootRepr { name, path } in pairs {
            let root = match path {
                Some(path) => RootConfig::Explicit(path),
                None => RootConfig::UseDefault,
            };
            map.map.insert(name, root);
        }
        Ok(map)
    }
}

impl RootMap {
    pub(super) fn default_root(&self, default_root_path: &TildePath) -> OptionalParam<Root> {
        let name = DEFAULT_ROOT_NAME.to_string();
        self.map
            .get(DEFAULT_ROOT_NAME)
            .cloned()
            .unwrap_or(RootConfig::UseDefault)
            .into_root(name, default_root_path)
    }

    pub(super) fn roots(
        &self,
        default_root_path: &TildePath,
    ) -> BTreeMap<String, OptionalParam<Root>> {
        let mut roots: BTreeMap<String, OptionalParam<Root>> = self
            .map
            .iter()
            .map(|(name, root)| {
                (
                    name.clone(),
                    root.clone().into_root(name.clone(), default_root_path),
                )
            })
            .collect();

        roots
            .entry(DEFAULT_ROOT_NAME.to_string())
            .or_insert_with(|| {
                RootConfig::UseDefault.into_root(DEFAULT_ROOT_NAME.to_string(), default_root_path)
            });

        roots
    }
}

#[derive(Debug, Clone)]
enum RootConfig {
    UseDefault,
    Explicit(TildePath),
}

impl RootConfig {
    fn into_root(self, name: String, default_root_path: &TildePath) -> OptionalParam<Root> {
        match self {
            Self::UseDefault => {
                let path = OptionalParam::new_default("root", default_root_path.clone());
                path.as_ref().map(|path| Root::new(name, path.into()))
            }
            Self::Explicit(path) => {
                let path = OptionalParam::new_explicit("root", path);
                path.as_ref().map(|path| Root::new(name, path.into()))
            }
        }
    }
}
