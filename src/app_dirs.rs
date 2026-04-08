use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};
use directories::BaseDirs;

#[derive(Debug, Clone)]
pub(crate) struct AppDirs {
    home_dir: PathBuf,
    config_dir: PathBuf,
    data_local_dir: PathBuf,
    cache_dir: PathBuf,
}

impl AppDirs {
    pub(crate) fn new(app_name: &str) -> Result<Self> {
        if let Some(home_dir) = env::var_os("SOUKO_INTEGRATION_TEST_HOME") {
            return Self::new_for_test(app_name, home_dir);
        }

        let base_dirs = BaseDirs::new().ok_or_else(|| eyre!("failed to get base directories"))?;
        let project_dirs = directories::ProjectDirs::from("", "", app_name)
            .ok_or_else(|| eyre!("failed to get project directories"))?;

        Ok(Self {
            home_dir: base_dirs.home_dir().to_owned(),
            config_dir: project_dirs.config_dir().to_owned(),
            data_local_dir: project_dirs.data_local_dir().to_owned(),
            cache_dir: project_dirs.cache_dir().to_owned(),
        })
    }

    pub(crate) fn new_for_test(app_name: &str, home_dir: impl Into<PathBuf>) -> Result<Self> {
        let home_dir = home_dir.into();
        if cfg!(target_os = "windows") {
            Ok(Self {
                config_dir: home_dir.join(format!(r"AppData\Roaming\{app_name}\config")),
                data_local_dir: home_dir.join(format!(r"AppData\Local\{app_name}\data")),
                cache_dir: home_dir.join(format!(r"AppData\Local\{app_name}\cache")),
                home_dir,
            })
        } else if cfg!(target_os = "linux") {
            Ok(Self {
                config_dir: home_dir.join(format!(".config/{app_name}")),
                data_local_dir: home_dir.join(format!(".local/share/{app_name}")),
                cache_dir: home_dir.join(format!(".cache/{app_name}")),
                home_dir,
            })
        } else if cfg!(target_os = "macos") {
            Ok(Self {
                config_dir: home_dir.join(format!("Library/Application Support/{app_name}")),
                data_local_dir: home_dir.join(format!("Library/Application Support/{app_name}")),
                cache_dir: home_dir.join(format!("Library/Caches/{app_name}")),
                home_dir,
            })
        } else {
            Err(eyre!("unsupported platform: {}", std::env::consts::OS))
        }
    }

    pub(crate) fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    pub(crate) fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub(crate) fn data_local_dir(&self) -> &Path {
        &self.data_local_dir
    }

    pub(crate) fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}
