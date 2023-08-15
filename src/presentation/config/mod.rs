use serde::Deserialize;

mod query;
mod root;

pub(crate) use self::{
    query::QueryConfig,
    root::{Root, RootMap},
};
use crate::domain::model::query::ParseOption;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(rename = "root")]
    root_map: RootMap,
    #[serde(rename = "query")]
    query_config: QueryConfig,
}

impl Config {
    pub(crate) fn root_map(&self) -> &RootMap {
        &self.root_map
    }

    pub(crate) fn query_parse_option(&self) -> ParseOption {
        self.query_config.clone().into()
    }
}
