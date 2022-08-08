use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::OnceCell;

static PROJECT_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

#[derive(Debug, Clone)]
pub(crate) struct ProjectDirs {
    config_dir: Option<PathBuf>,
    data_local_dir: Option<PathBuf>,
    inner: directories::ProjectDirs,
}

impl ProjectDirs {
    pub(crate) fn init() -> Result<()> {
        let project_dirs = ProjectDirs::new()?;
        PROJECT_DIRS
            .set(project_dirs)
            .expect("BUG: faield to set project directories");
        Ok(())
    }

    pub(crate) fn get() -> &'static Self {
        PROJECT_DIRS
            .get()
            .expect("BUG: project_dirs::get() called before project_dirs::init()")
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

        Ok(Self {
            config_dir,
            data_local_dir,
            inner,
        })
    }

    pub(crate) fn config_dir(&self) -> &Path {
        self.config_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.config_dir())
    }

    pub(crate) fn data_local_dir(&self) -> &Path {
        self.data_local_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.data_local_dir())
    }
}
