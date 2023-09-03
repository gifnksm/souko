use std::collections::BTreeMap;

use serde::Deserialize;

mod query;
mod root;

use self::{query::QueryConfig, root::RootMap};
use super::model::optional_param::OptionalParam;
use crate::domain::model::{query::ParseOption, root::Root};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Config {
    #[serde(rename = "root", default)]
    root_map: RootMap,
    #[serde(rename = "query", default)]
    query_config: QueryConfig,
}

impl Config {
    pub(super) fn roots(&self) -> BTreeMap<String, OptionalParam<Root>> {
        self.root_map.roots()
    }

    pub(super) fn default_root(&self) -> OptionalParam<Root> {
        self.root_map.default_root()
    }

    pub(super) fn query_parse_option(&self) -> ParseOption {
        self.query_config.clone().into()
    }
}
