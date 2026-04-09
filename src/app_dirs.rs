use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, ensure, eyre};
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

        Self::from_dirs(
            base_dirs.home_dir(),
            project_dirs.config_dir(),
            project_dirs.data_local_dir(),
            project_dirs.cache_dir(),
        )
    }

    pub(crate) fn new_for_test(app_name: &str, home_dir: impl Into<PathBuf>) -> Result<Self> {
        let home_dir = home_dir.into();
        let (config_dir, data_local_dir, cache_dir) = if cfg!(target_os = "windows") {
            (
                home_dir.join(format!(r"AppData\Roaming\{app_name}\config")),
                home_dir.join(format!(r"AppData\Local\{app_name}\data")),
                home_dir.join(format!(r"AppData\Local\{app_name}\cache")),
            )
        } else if cfg!(target_os = "linux") {
            (
                home_dir.join(format!(".config/{app_name}")),
                home_dir.join(format!(".local/share/{app_name}")),
                home_dir.join(format!(".cache/{app_name}")),
            )
        } else if cfg!(target_os = "macos") {
            (
                home_dir.join(format!("Library/Application Support/{app_name}")),
                home_dir.join(format!("Library/Application Support/{app_name}")),
                home_dir.join(format!("Library/Caches/{app_name}")),
            )
        } else {
            return Err(eyre!("unsupported platform: {}", std::env::consts::OS));
        };
        Self::from_dirs(home_dir, config_dir, data_local_dir, cache_dir)
    }

    fn from_dirs<P, Q, R, S>(
        home_dir: P,
        config_dir: Q,
        data_local_dir: R,
        cache_dir: S,
    ) -> Result<Self>
    where
        P: AsRef<Path> + Into<PathBuf>,
        Q: AsRef<Path> + Into<PathBuf>,
        R: AsRef<Path> + Into<PathBuf>,
        S: AsRef<Path> + Into<PathBuf>,
    {
        Ok(Self {
            home_dir: ensure_absolute_path("home_dir", home_dir)?.into(),
            config_dir: ensure_absolute_path("config_dir", config_dir)?.into(),
            data_local_dir: ensure_absolute_path("data_local_dir", data_local_dir)?.into(),
            cache_dir: ensure_absolute_path("cache_dir", cache_dir)?.into(),
        })
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

fn ensure_absolute_path<P>(name: &str, path: P) -> Result<P>
where
    P: AsRef<Path>,
{
    ensure!(
        path.as_ref().is_absolute(),
        "{name} is not an absolute path: {}",
        path.as_ref().display()
    );
    Ok(path)
}
