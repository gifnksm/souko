use serde::Deserialize;

mod query;
mod root;

use self::{query::QueryConfig, root::RootMap};
use super::util::{optional_param::OptionalParam, tilde_path::TildePath};
use crate::domain::model::{query::ParseOption, root::RootSpec};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Config {
    #[serde(rename = "root")]
    root_map: RootMap,
    #[serde(rename = "query")]
    query_config: QueryConfig,
}

impl Config {
    pub(super) fn root_specs(&self) -> Vec<OptionalParam<RootSpec>> {
        self.root_map.specs()
    }

    pub(super) fn default_root_path(&self) -> &OptionalParam<TildePath> {
        self.root_map.default_root_path()
    }

    pub(super) fn query_parse_option(&self) -> ParseOption {
        self.query_config.clone().into()
    }
}
