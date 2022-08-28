use std::collections::HashMap;

use serde::Deserialize;

use crate::{query, Scheme, Template};

mod root;

pub(crate) use root::{Root, RootMap};

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct Config {
    #[serde(rename = "root")]
    root_map: RootMap,
    default_scheme: Option<Scheme>,
    #[serde(default)]
    scheme_alias: HashMap<Scheme, Scheme>,
    #[serde(default)]
    custom_scheme: HashMap<Scheme, Template>,
}

impl<'a> Config {
    pub(crate) fn root_map(&self) -> &RootMap {
        &self.root_map
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
