use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::OnceCell;

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

#[derive(Debug, Clone)]
pub(in super::super) struct ProjectDirs {
    config_dir: Option<PathBuf>,
    data_local_dir: Option<PathBuf>,
    cache_dir: Option<PathBuf>,
    inner: directories::ProjectDirs,
}

impl ProjectDirs {
    pub(in super::super) fn init() -> Result<()> {
        let project_dirs = ProjectDirs::new()?;
        PROJECT_DIRS
            .set(project_dirs)
            .expect("BUG: failed to set project directories");
        Ok(())
    }

    pub(in super::super) fn get() -> &'static Self {
        PROJECT_DIRS
            .get()
            .expect("BUG: ProjectDirs::get() called before ProjectDirs::init()")
    }

    fn new() -> Result<Self> {
        let integration_test = env::var_os("SOUKO_INTEGRATION_TEST").is_some();
        let inner = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or_else(|| eyre!("failed to get project directories"))?;

        let config_dir = (integration_test && cfg!(target_os = "windows")).then(|| {
            let home = env::var_os("HOME").expect("BUG: missing HOME");
            Path::new(&home).join(r"AppData\Roaming\souko\config")
        });
        let data_local_dir = (integration_test && cfg!(target_os = "windows")).then(|| {
            let home = env::var_os("HOME").expect("BUG: missing HOME");
            Path::new(&home).join(r"AppData\Local\souko\data")
        });
        let cache_dir = (integration_test && cfg!(target_os = "windows")).then(|| {
            let home = env::var_os("HOME").expect("BUG: missing HOME");
            Path::new(&home).join(r"AppData\Local\souko\cache")
        });

        Ok(Self {
            config_dir,
            data_local_dir,
            cache_dir,
            inner,
        })
    }

    pub(in super::super) fn config_dir(&self) -> &Path {
        self.config_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.config_dir())
    }

    pub(in super::super) fn data_local_dir(&self) -> &Path {
        self.data_local_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.data_local_dir())
    }

    pub(in super::super) fn cache_dir(&self) -> &Path {
        self.cache_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.cache_dir())
    }
}
