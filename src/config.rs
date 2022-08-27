use std::collections::HashMap;

use serde::Deserialize;

use crate::{query, Scheme, Template};

mod root;

pub(crate) use root::Root;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    #[serde(
        rename = "root",
        deserialize_with = "root::deserialize",
        default = "root::default"
    )]
    root_map: HashMap<String, Root>,
    default_scheme: Option<Scheme>,
    #[serde(default)]
    scheme_alias: HashMap<Scheme, Scheme>,
    #[serde(default)]
    custom_scheme: HashMap<Scheme, Template>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_map: root::default(),
            default_scheme: None,
            scheme_alias: HashMap::new(),
            custom_scheme: HashMap::new(),
        }
    }
}

impl<'a> Config {
    pub(crate) fn root_map(&self) -> &HashMap<String, Root> {
        &self.root_map
    }

    pub(crate) fn default_root(&self) -> &Root {
        self.root_map.get("default").unwrap()
    }

    pub(crate) fn query_config(&'a self) -> query::Config {
        let default_scheme = self
            .default_scheme
            .clone()
            .unwrap_or_else(|| "gh".parse().expect("BUG: invalid default scheme"));
        let default_scheme = Some(default_scheme);
        let mut scheme_alias = self.scheme_alias.clone();
        let mut custom_scheme = self.custom_scheme.clone();

        let predefined_alias = [("gh", "github"), ("gl", "gitlab")];
        for (alias, scheme) in predefined_alias.iter() {
            let alias = alias.parse().expect("BUG: invalid alias");
            let scheme = scheme.parse().expect("BUG: invalid scheme");
            scheme_alias.entry(alias).or_insert_with(|| scheme);
        }

        let predefined_scheme = [
            ("github", "https://github.com/{path}.git"),
            ("gitlab", "https://gitlab.com/{path}.git"),
        ];
        for (scheme, template) in predefined_scheme.iter() {
            let scheme = scheme.parse().expect("BUG: invalid scheme");
            let template = template.parse().expect("BUG: invalid template");
            custom_scheme.entry(scheme).or_insert_with(|| template);
        }

        query::Config {
            default_scheme,
            scheme_alias,
            custom_scheme,
        }
    }
}
