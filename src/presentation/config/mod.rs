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

#[cfg(test)]
mod tests {
    use super::*;

    // We intentionally deserialize only the `[query]` subtree in these tests.
    //
    // Why not deserialize top-level `Config`?
    // - `Config` includes `root_map`.
    // - `root_map` default construction depends on `ProjectDirs::get()`, which
    //   requires global initialization and is unrelated to query-map regression coverage.
    //
    // This wrapper keeps the test focused on the real failure mode:
    // deserializing nested `[query]` / `[query.custom_scheme]` when some query
    // fields are omitted.
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct QueryOnlyWrapper {
        #[serde(rename = "query")]
        query_config: QueryConfig,
    }

    impl QueryOnlyWrapper {
        fn into_parse_option(self) -> ParseOption {
            self.query_config.into()
        }
    }

    #[test]
    fn deserialize_query_without_scheme_alias_applies_overrides_and_merges_defaults() {
        let input = r#"
            [query]
            default_scheme = "gl"

            [query.custom_scheme]
            github = "https://example.com/{path}"
            sourcehut = "https://git.sr.ht/~{path}"
        "#;

        let wrapper: QueryOnlyWrapper = toml_edit::de::from_str(input).unwrap();
        let option = wrapper.into_parse_option();

        assert_eq!(option.default_scheme, Some("gl".parse().unwrap()));
        assert_eq!(
            option.scheme_alias.get("gh"),
            Some(&"github".parse().unwrap())
        );
        assert_eq!(
            option.scheme_alias.get("gl"),
            Some(&"gitlab".parse().unwrap())
        );

        let query =
            crate::domain::model::query::Query::parse("github:gifnksm/souko", &option).unwrap();
        assert_eq!(query.url().as_str(), "https://example.com/gifnksm/souko");

        let query =
            crate::domain::model::query::Query::parse("sourcehut:gifnksm/souko", &option).unwrap();
        assert_eq!(query.url().as_str(), "https://git.sr.ht/~gifnksm/souko");

        let query = crate::domain::model::query::Query::parse("gl:gifnksm/souko", &option).unwrap();
        assert_eq!(query.url().as_str(), "https://gitlab.com/gifnksm/souko.git");
    }

    #[test]
    fn deserialize_query_without_custom_scheme_keeps_default_schemes() {
        let input = r#"
            [query]
            default_scheme = "gh"
        "#;

        let wrapper: QueryOnlyWrapper = toml_edit::de::from_str(input).unwrap();
        let option = wrapper.into_parse_option();

        assert_eq!(option.default_scheme, Some("gh".parse().unwrap()));

        let query = crate::domain::model::query::Query::parse("gh:gifnksm/souko", &option).unwrap();
        assert_eq!(query.url().as_str(), "https://github.com/gifnksm/souko.git");

        let query = crate::domain::model::query::Query::parse("gl:gifnksm/souko", &option).unwrap();
        assert_eq!(query.url().as_str(), "https://gitlab.com/gifnksm/souko.git");
    }
}
