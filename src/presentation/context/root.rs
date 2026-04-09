use std::collections::BTreeMap;

use color_eyre::eyre::eyre;

use crate::{
    app_dirs::AppDirs,
    domain::model::{pretty_path::PrettyPath, root::Root},
    presentation::{
        config::{DEFAULT_ROOT_NAME, RootConfig},
        model::{
            app_param::{AppParam, AppParamSource},
            unresolved_path::UnresolvedPath,
        },
    },
};

#[derive(Debug)]
pub(in crate::presentation) struct RootContextMap {
    map: BTreeMap<String, AppParam<RootContext>>,
}

impl RootContextMap {
    pub(in crate::presentation) fn new(config: &[RootConfig], app_dirs: &AppDirs) -> Self {
        let mut map: BTreeMap<String, AppParam<RootContext>> = config
            .iter()
            .map(|root_config| {
                // A root entry present in config is treated as coming from the configuration
                // file even when its path is omitted and resolved to the default path. Only the
                // synthesized fallback `default` root (added when no such entry exists in config
                // at all) is treated as an implicit default.
                let source = AppParamSource::ConfigurationFile;
                let value = RootContext::from_config(root_config, app_dirs);
                (
                    root_config.name.clone(),
                    AppParam::new("root", source, value),
                )
            })
            .collect();

        map.entry(DEFAULT_ROOT_NAME.to_owned()).or_insert_with(|| {
            let source = AppParamSource::ImplicitDefault;
            let value = RootContext::from_config(&RootConfig::default_root(), app_dirs);
            AppParam::new("root", source, value)
        });

        Self { map }
    }

    pub(in crate::presentation) fn default_root(&self) -> &AppParam<RootContext> {
        self.map.get(DEFAULT_ROOT_NAME).unwrap()
    }

    pub(in crate::presentation) fn root_by_name(
        &self,
        name: &str,
    ) -> Option<&AppParam<RootContext>> {
        self.map.get(name)
    }

    pub(in crate::presentation) fn root_by_name_or_err(
        &self,
        name: &str,
    ) -> Result<&AppParam<RootContext>, color_eyre::Report> {
        self.root_by_name(name)
            .ok_or_else(|| eyre!("root `{name}` not found in config file"))
    }

    pub(in crate::presentation) fn all_roots(
        &self,
    ) -> impl Iterator<Item = &AppParam<RootContext>> {
        self.map.values()
    }
}

fn default_path(app_dirs: &AppDirs) -> PrettyPath {
    UnresolvedPath::new(app_dirs.data_local_dir().join("root"))
        .normalize_with_home(app_dirs.home_dir())
}

#[derive(Debug, Clone)]
pub(in crate::presentation) struct RootContext {
    root: Root,
    visit_hidden_dirs: bool,
    visit_repo_subdirs: bool,
    include_bare_repo: bool,
}

impl RootContext {
    fn from_config(root_config: &RootConfig, app_dirs: &AppDirs) -> Self {
        let path = root_config
            .path
            .clone()
            .map(|path| path.normalize_with_home(app_dirs.home_dir()))
            .unwrap_or_else(|| default_path(app_dirs));
        Self {
            root: Root::new(root_config.name.clone(), path),
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
    use tempfile::TempDir;

    use crate::{domain::model::path_like::PathLike as _, presentation::config::Config};

    use super::*;

    #[test]
    fn default_config_resolves_default_root_from_injected_app_dirs() {
        let home = TempDir::new().unwrap();
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), home.path()).unwrap();
        let config = Config::default();

        let root_ctx = RootContextMap::new(&config.roots, &app_dirs);

        let default_root = root_ctx.default_root();
        assert_eq!(default_root.value().name(), "default");
        assert_eq!(
            default_root.value().path().as_real_path(),
            &app_dirs.data_local_dir().join("root")
        );
    }

    #[test]
    fn explicit_default_root_in_config_overrides_injected_default_root_path() {
        let home = TempDir::new().unwrap();
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), home.path()).unwrap();
        let config: Config = toml_edit::de::from_str(
            r#"
            [[root]]
            name = "default"
            path = "/tmp/custom-root"
            "#,
        )
        .unwrap();

        let root_ctx = RootContextMap::new(&config.roots, &app_dirs);

        let default_root = root_ctx.default_root();
        assert_eq!(default_root.value().name(), "default");
        assert_eq!(
            default_root.value().path().as_real_path(),
            std::path::Path::new("/tmp/custom-root")
        );
    }

    #[test]
    fn configuration_file_root_entry_with_omitted_path_remains_configuration_file() {
        let home = TempDir::new().unwrap();
        let app_dirs = AppDirs::new_for_test(env!("CARGO_BIN_NAME"), home.path()).unwrap();
        let config: Config = toml_edit::de::from_str(
            r#"
            [[root]]
            name = "foo"
            "#,
        )
        .unwrap();

        let root_ctx = RootContextMap::new(&config.roots, &app_dirs);

        let foo_root = root_ctx.root_by_name("foo").unwrap();
        assert_eq!(foo_root.value().name(), "foo");
        assert!(foo_root.source().is_configuration_file());
        assert_eq!(
            foo_root.value().path().as_real_path(),
            &app_dirs.data_local_dir().join("root")
        );

        let default_root = root_ctx.default_root();
        assert!(default_root.source().is_implicit_default());
        assert_eq!(default_root.value().name(), "default");
    }
}
