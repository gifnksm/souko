use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::{query, OptionalParam, ProjectDirs, Template};

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Config {
    default_root: Option<PathBuf>,
    default_scheme: Option<query::Scheme>,
    scheme_alias: HashMap<query::Scheme, query::Scheme>,
    custom_scheme: HashMap<query::Scheme, Template>,
}

impl<'a> Config {
    pub(crate) fn default_root(&'a self) -> OptionalParam<'a, PathBuf> {
        OptionalParam::new("root", &self.default_root, || {
            ProjectDirs::get().data_local_dir().join("root")
        })
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