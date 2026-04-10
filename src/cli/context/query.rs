use std::{collections::HashMap, str::FromStr as _};

use crate::{
    cli::config::QueryConfig,
    domain::model::{query::ParseOption, scheme::Scheme, template::Template},
};

#[derive(Debug)]
pub(in crate::cli) struct QueryContext {
    parse_option: ParseOption,
}

fn predefined_aliases() -> HashMap<Scheme, Scheme> {
    [("gh", "github"), ("gl", "gitlab")]
        .into_iter()
        .map(|(src, dst)| {
            let src = Scheme::from_str(src).unwrap();
            let dst = Scheme::from_str(dst).unwrap();
            (src, dst)
        })
        .collect()
}

fn predefined_custom_schemes() -> HashMap<Scheme, Template> {
    [
        ("github", "https://github.com/{path}.git"),
        ("gitlab", "https://gitlab.com/{path}.git"),
    ]
    .into_iter()
    .map(|(scheme, template)| {
        let scheme = Scheme::from_str(scheme).unwrap();
        let template = Template::from_str(template).unwrap();
        (scheme, template)
    })
    .collect()
}

impl QueryContext {
    pub(in crate::cli) fn from_config(config: &QueryConfig) -> Self {
        let mut parse_option = ParseOption {
            default_scheme: Some(config.default_scheme.clone()),
            scheme_alias: predefined_aliases(),
            custom_scheme: predefined_custom_schemes(),
        };
        // existing key will be overwritten by the values from config file
        // see `std::iter::Extend`.
        parse_option.scheme_alias.extend(
            config
                .scheme_alias
                .iter()
                .map(|(src, dst)| (src.clone(), dst.clone())),
        );
        parse_option.custom_scheme.extend(
            config
                .custom_scheme
                .iter()
                .map(|(scheme, template)| (scheme.clone(), template.clone())),
        );

        Self { parse_option }
    }

    pub(in crate::cli) fn parse_option(&self) -> &ParseOption {
        &self.parse_option
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::config::Config;

    use super::*;

    #[test]
    fn deserialize_query_without_scheme_alias_applies_overrides_and_merges_defaults() {
        let input = r#"
            [query]
            default_scheme = "gl"

            [query.custom_scheme]
            github = "https://example.com/{path}"
            sourcehut = "https://git.sr.ht/~{path}"
        "#;

        let config: Config = toml_edit::de::from_str(input).unwrap();
        let query = QueryContext::from_config(&config.query);
        let option = query.parse_option();

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
            crate::domain::model::query::Query::parse("github:gifnksm/souko", option).unwrap();
        assert_eq!(query.url().as_str(), "https://example.com/gifnksm/souko");

        let query =
            crate::domain::model::query::Query::parse("sourcehut:gifnksm/souko", option).unwrap();
        assert_eq!(query.url().as_str(), "https://git.sr.ht/~gifnksm/souko");

        let query = crate::domain::model::query::Query::parse("gl:gifnksm/souko", option).unwrap();
        assert_eq!(query.url().as_str(), "https://gitlab.com/gifnksm/souko.git");
    }

    #[test]
    fn deserialize_query_without_custom_scheme_keeps_default_schemes() {
        let input = r#"
            [query]
            default_scheme = "gh"
        "#;

        let config: Config = toml_edit::de::from_str(input).unwrap();
        let query = QueryContext::from_config(&config.query);
        let option = query.parse_option();

        assert_eq!(option.default_scheme, Some("gh".parse().unwrap()));

        let query = crate::domain::model::query::Query::parse("gh:gifnksm/souko", option).unwrap();
        assert_eq!(query.url().as_str(), "https://github.com/gifnksm/souko.git");

        let query = crate::domain::model::query::Query::parse("gl:gifnksm/souko", option).unwrap();
        assert_eq!(query.url().as_str(), "https://gitlab.com/gifnksm/souko.git");
    }
}
