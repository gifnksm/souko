use serde::Deserialize;

mod query;
mod root;

pub(crate) use self::{
    query::QueryConfig,
    root::{Root, RootMap},
};

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

    pub(crate) fn query_config(&self) -> &QueryConfig {
        &self.query_config
    }
}
