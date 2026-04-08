use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};

#[derive(Debug, Clone)]
pub(crate) struct AppDirs {
    config_dir: PathBuf,
    data_local_dir: PathBuf,
    cache_dir: PathBuf,
}

impl AppDirs {
    pub(crate) fn new(app_name: &str) -> Result<Self> {
        if let Some(home) = env::var_os("SOUKO_INTEGRATION_TEST_HOME") {
            let home = Path::new(&home);
            return Self::new_for_test(app_name, home);
        }

        let project_dirs = directories::ProjectDirs::from("", "", app_name)
            .ok_or_else(|| eyre!("failed to get project directories"))?;

        Ok(Self {
            config_dir: project_dirs.config_dir().to_owned(),
            data_local_dir: project_dirs.data_local_dir().to_owned(),
            cache_dir: project_dirs.cache_dir().to_owned(),
        })
    }

    pub(crate) fn new_for_test(app_name: &str, home: impl Into<PathBuf>) -> Result<Self> {
        let home = home.into();
        if cfg!(target_os = "windows") {
            Ok(Self {
                config_dir: home.join(format!(r"AppData\Roaming\{app_name}\config")),
                data_local_dir: home.join(format!(r"AppData\Local\{app_name}\data")),
                cache_dir: home.join(format!(r"AppData\Local\{app_name}\cache")),
            })
        } else if cfg!(target_os = "linux") {
            Ok(Self {
                config_dir: home.join(format!(".config/{app_name}")),
                data_local_dir: home.join(format!(".local/share/{app_name}")),
                cache_dir: home.join(format!(".cache/{app_name}")),
            })
        } else if cfg!(target_os = "macos") {
            Ok(Self {
                config_dir: home.join(format!("Library/Application Support/{app_name}")),
                data_local_dir: home.join(format!("Library/Application Support/{app_name}")),
                cache_dir: home.join(format!("Library/Caches/{app_name}")),
            })
        } else {
            Err(eyre!("unsupported platform: {}", std::env::consts::OS))
        }
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
