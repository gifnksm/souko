use std::{collections::HashMap, str::FromStr as _};

use serde::Deserialize;

use super::model::tilde_path::TildePath;
use crate::domain::model::{scheme::Scheme, template::Template};

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
    pub(in crate::presentation) path: Option<TildePath>,
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
