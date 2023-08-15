use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::model::{query::ParseOption, scheme::Scheme, template::Template};

#[derive(Debug, Clone)]
pub(crate) struct QueryConfig {
    pub(crate) default_scheme: Option<Scheme>,
    pub(crate) scheme_alias: HashMap<Scheme, Scheme>,
    pub(crate) custom_scheme: HashMap<Scheme, Template>,
}

impl Default for QueryConfig {
    fn default() -> Self {
        let predefined_alias = [("gh", "github"), ("gl", "gitlab")];
        let predefined_scheme = [
            ("github", "https://github.com/{path}.git"),
            ("gitlab", "https://gitlab.com/{path}.git"),
        ];

        let default_scheme = Some("gh".parse().expect("BUG: invalid default scheme"));
        let scheme_alias = predefined_alias
            .into_iter()
            .map(|(alias, scheme)| {
                let alias = alias.parse().expect("BUG: invalid alias");
                let scheme = scheme.parse().expect("BUG: invalid scheme");
                (alias, scheme)
            })
            .collect();
        let custom_scheme = predefined_scheme
            .into_iter()
            .map(|(scheme, template)| {
                let scheme = scheme.parse().expect("BUG: invalid scheme");
                let template = template.parse().expect("BUG: invalid template");
                (scheme, template)
            })
            .collect();

        Self {
            default_scheme,
            scheme_alias,
            custom_scheme,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigRepr {
    default_scheme: Option<Scheme>,
    scheme_alias: HashMap<Scheme, Scheme>,
    custom_scheme: HashMap<Scheme, Template>,
}

impl<'de> Deserialize<'de> for QueryConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ConfigRepr {
            default_scheme,
            scheme_alias,
            custom_scheme,
        } = ConfigRepr::deserialize(deserializer)?;

        let mut config = Self::default();
        if let Some(scheme) = default_scheme {
            config.default_scheme = Some(scheme);
        }

        // existing key will be overwritten by the values from config file
        // see `std::iter::Extend`.
        config.scheme_alias.extend(scheme_alias);
        config.custom_scheme.extend(custom_scheme);

        Ok(config)
    }
}

impl From<QueryConfig> for ParseOption {
    fn from(value: QueryConfig) -> Self {
        let QueryConfig {
            default_scheme,
            scheme_alias,
            custom_scheme,
        } = value;
        Self {
            default_scheme,
            scheme_alias,
            custom_scheme,
        }
    }
}
