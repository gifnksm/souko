use std::{collections::HashMap, str::FromStr as _};

use serde::Deserialize;

use super::model::unresolved_path::UnresolvedPath;
use crate::domain::model::{scheme::Scheme, template::Template};

pub(in crate::presentation) const DEFAULT_ROOT_NAME: &str = "default";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::presentation) struct Config {
    #[serde(rename = "root", default)]
    pub(in crate::presentation) roots: Vec<RootConfig>,
    #[serde(default)]
    pub(in crate::presentation) query: QueryConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::presentation) struct RootConfig {
    pub(in crate::presentation) name: String,
    #[serde(default)]
    pub(in crate::presentation) path: Option<UnresolvedPath>,
    #[serde(default)]
    pub(in crate::presentation) visit_hidden_dirs: bool,
    #[serde(default)]
    pub(in crate::presentation) visit_repo_subdirs: bool,
    #[serde(default)]
    pub(in crate::presentation) include_bare_repo: bool,
}

impl RootConfig {
    pub(in crate::presentation) fn default_root() -> Self {
        Self {
            name: DEFAULT_ROOT_NAME.to_owned(),
            path: None,
            visit_hidden_dirs: false,
            visit_repo_subdirs: false,
            include_bare_repo: false,
        }
    }
}

fn default_scheme() -> Scheme {
    Scheme::from_str("gh").unwrap()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::presentation) struct QueryConfig {
    #[serde(default = "default_scheme")]
    pub(in crate::presentation) default_scheme: Scheme,
    #[serde(default)]
    pub(in crate::presentation) scheme_alias: HashMap<Scheme, Scheme>,
    #[serde(default)]
    pub(in crate::presentation) custom_scheme: HashMap<Scheme, Template>,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            default_scheme: default_scheme(),
            scheme_alias: Default::default(),
            custom_scheme: Default::default(),
        }
    }
}
