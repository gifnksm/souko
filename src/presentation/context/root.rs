use std::collections::BTreeMap;

use crate::{
    domain::model::{pretty_path::PrettyPath, root::Root},
    presentation::{
        config::{DEFAULT_ROOT_NAME, RootConfig},
        model::{optional_param::OptionalParam, tilde_path::TildePath},
    },
    project_dirs::ProjectDirs,
};

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
                RootContext::from_config(&RootConfig::default_root(), project_dirs),
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

fn default_path(project_dirs: &ProjectDirs) -> TildePath {
    TildePath::from_real_path(project_dirs.data_local_dir().join("root"))
}

#[derive(Debug, Clone)]
pub(in crate::presentation) struct RootContext {
    root: Root,
    visit_hidden_dirs: bool,
    visit_repo_subdirs: bool,
    include_bare_repo: bool,
}

impl RootContext {
    fn from_config(root_config: &RootConfig, project_dirs: &ProjectDirs) -> Self {
        let path = root_config
            .path
            .clone()
            .unwrap_or_else(|| default_path(project_dirs));
        Self {
            root: Root::new(root_config.name.clone(), path.into()),
            visit_hidden_dirs: root_config.visit_hidden_dirs,
            visit_repo_subdirs: root_config.visit_repo_subdirs,
            include_bare_repo: root_config.include_bare_repo,
        }
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

    pub(in crate::presentation) fn visit_hidden_dirs(&self) -> bool {
        self.visit_hidden_dirs
    }

    pub(in crate::presentation) fn include_bare_repo(&self) -> bool {
        self.include_bare_repo
    }

    pub(in crate::presentation) fn visit_repo_subdirs(&self) -> bool {
        self.visit_repo_subdirs
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

    #[test]
    fn explicit_root_entry_with_omitted_path_remains_explicit() {
        let project_dirs = ProjectDirs::new_for_test(
            "target/test-config-dir-implicit-path-root",
            "target/test-data-local-dir-implicit-path-root",
            "target/test-cache-dir-implicit-path-root",
        )
        .unwrap();
        let config: Config = toml_edit::de::from_str(
            r#"
            [[root]]
            name = "foo"
            "#,
        )
        .unwrap();

        let root_ctx = RootContextMap::new(&config.roots, &project_dirs);

        let foo_root = root_ctx.root_by_name("foo").unwrap();
        assert!(foo_root.is_explicit());
        assert_eq!(foo_root.value().name(), "foo");
        assert_eq!(
            foo_root.value().path().as_real_path(),
            &project_dirs.data_local_dir().join("root")
        );

        let default_root = root_ctx.default_root();
        assert!(default_root.is_default());
        assert_eq!(default_root.value().name(), "default");
    }
}
