use std::collections::BTreeMap;

use crate::{
    domain::model::{pretty_path::PrettyPath, root::Root},
    presentation::{
        config::RootConfig,
        model::{optional_param::OptionalParam, tilde_path::TildePath},
    },
    project_dirs::ProjectDirs,
};

const DEFAULT_ROOT_NAME: &str = "default";

#[derive(Debug)]
pub(in crate::presentation) struct RootContextMap {
    map: BTreeMap<String, OptionalParam<RootContext>>,
}

impl RootContextMap {
    pub(in crate::presentation) fn new(config: &[RootConfig], project_dirs: &ProjectDirs) -> Self {
        let mut map: BTreeMap<String, OptionalParam<RootContext>> = config
            .iter()
            .map(|root_config| {
                // A root entry present in config is treated as explicit even when its path is
                // omitted and resolved to the default path. Only the synthesized fallback
                // `default` root (added when no such entry exists in config at all) is treated as
                // non-explicit.
                (
                    root_config.name.clone(),
                    OptionalParam::new_explicit(
                        "root",
                        RootContext::from_config(root_config, project_dirs),
                    ),
                )
            })
            .collect();

        map.entry(DEFAULT_ROOT_NAME.to_owned()).or_insert_with(|| {
            OptionalParam::new_default(
                "root",
                RootContext::with_default_path(DEFAULT_ROOT_NAME.to_owned(), project_dirs),
            )
        });

        Self { map }
    }

    pub(in crate::presentation) fn default_root(&self) -> &OptionalParam<RootContext> {
        self.map.get(DEFAULT_ROOT_NAME).unwrap()
    }

    pub(in crate::presentation) fn root_by_name(
        &self,
        name: &str,
    ) -> Option<&OptionalParam<RootContext>> {
        self.map.get(name)
    }

    pub(in crate::presentation) fn root_by_name_or_err(
        &self,
        name: &str,
    ) -> Result<&OptionalParam<RootContext>, color_eyre::Report> {
        self.root_by_name(name)
            .ok_or_else(|| color_eyre::eyre::eyre!("root `{name}` not found in config file"))
    }

    pub(in crate::presentation) fn all_roots(
        &self,
    ) -> impl Iterator<Item = &OptionalParam<RootContext>> {
        self.map.values()
    }
}

#[derive(Debug, Clone)]
pub(in crate::presentation) struct RootContext {
    root: Root,
}

impl RootContext {
    fn from_config(root_config: &RootConfig, project_dirs: &ProjectDirs) -> Self {
        match &root_config.path {
            Some(path) => Self::new(root_config.name.clone(), path.clone()),
            None => Self::with_default_path(root_config.name.clone(), project_dirs),
        }
    }

    fn new(name: String, path: TildePath) -> Self {
        let root = Root::new(name, path.into());
        Self { root }
    }

    fn with_default_path(name: String, project_dirs: &ProjectDirs) -> Self {
        let default_root_path = project_dirs.data_local_dir().join("root");
        let path = TildePath::from_real_path(default_root_path);
        Self::new(name, path)
    }

    pub(in crate::presentation) fn root(&self) -> &Root {
        &self.root
    }

    #[cfg(test)]
    pub(in crate::presentation) fn name(&self) -> &str {
        self.root.name()
    }

    pub(in crate::presentation) fn path(&self) -> &PrettyPath {
        self.root.path()
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::model::path_like::PathLike as _, presentation::config::Config};

    use super::*;

    #[test]
    fn default_config_resolves_default_root_from_injected_project_dirs() {
        let project_dirs = ProjectDirs::new_for_test(
            "target/test-config-dir-default-root",
            "target/test-data-local-dir-default-root",
            "target/test-cache-dir-default-root",
        )
        .unwrap();
        let config = Config::default();

        let root_ctx = RootContextMap::new(&config.roots, &project_dirs);

        let default_root = root_ctx.default_root();
        assert_eq!(default_root.value().name(), "default");
        assert_eq!(
            default_root.value().path().as_real_path(),
            &project_dirs.data_local_dir().join("root")
        );
    }

    #[test]
    fn explicit_default_root_in_config_overrides_injected_default_root_path() {
        let project_dirs = ProjectDirs::new_for_test(
            "target/test-config-dir-explicit-root",
            "target/test-data-local-dir-explicit-root",
            "target/test-cache-dir-explicit-root",
        )
        .unwrap();
        let config: Config = toml_edit::de::from_str(
            r#"
            [[root]]
            name = "default"
            path = "/tmp/custom-root"
            "#,
        )
        .unwrap();

        let root_ctx = RootContextMap::new(&config.roots, &project_dirs);

        let default_root = root_ctx.default_root();
        assert_eq!(default_root.value().name(), "default");
        assert_eq!(
            default_root.value().path().as_real_path(),
            std::path::Path::new("/tmp/custom-root")
        );
    }
}
