use serde::Deserialize;

mod query;
mod root;

use self::{query::QueryConfig, root::RootMap};
use super::util::optional_param::OptionalParam;
use crate::domain::model::{
    query::ParseOption,
    root::{Root, RootSpec},
};

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

    pub(super) fn default_root(&self) -> Root {
        self.root_map.default_root()
    }

    pub(super) fn query_parse_option(&self) -> ParseOption {
        self.query_config.clone().into()
    }
}
