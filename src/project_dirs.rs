use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};

#[derive(Debug, Clone)]
pub(crate) struct ProjectDirs {
    config_dir: Option<PathBuf>,
    data_local_dir: Option<PathBuf>,
    cache_dir: Option<PathBuf>,
    inner: directories::ProjectDirs,
}

impl ProjectDirs {
    pub(crate) fn new() -> Result<Self> {
        let integration_test_home = env::var_os("SOUKO_INTEGRATION_TEST_HOME");
        let app_name = env!("CARGO_PKG_NAME");
        let inner = directories::ProjectDirs::from("", "", app_name)
            .ok_or_else(|| eyre!("failed to get project directories"))?;

        let overridden_dirs = integration_test_home.and_then(|home| {
            if cfg!(target_os = "windows") {
                Some((
                    Path::new(&home).join(format!(r"AppData\Roaming\{app_name}\config")),
                    Path::new(&home).join(format!(r"AppData\Local\{app_name}\data")),
                    Path::new(&home).join(format!(r"AppData\Local\{app_name}\cache")),
                ))
            } else if cfg!(target_os = "linux") {
                Some((
                    Path::new(&home).join(format!(".config/{app_name}")),
                    Path::new(&home).join(format!(".local/share/{app_name}")),
                    Path::new(&home).join(format!(".cache/{app_name}")),
                ))
            } else if cfg!(target_os = "macos") {
                Some((
                    Path::new(&home).join(format!("Library/Application Support/{app_name}")),
                    Path::new(&home).join(format!("Library/Application Support/{app_name}")),
                    Path::new(&home).join(format!("Library/Caches/{app_name}")),
                ))
            } else {
                None
            }
        });

        if let Some((config_dir, data_local_dir, cache_dir)) = overridden_dirs {
            return Ok(Self {
                config_dir: Some(config_dir),
                data_local_dir: Some(data_local_dir),
                cache_dir: Some(cache_dir),
                inner,
            });
        }

        Ok(Self {
            config_dir: None,
            data_local_dir: None,
            cache_dir: None,
            inner,
        })
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(
        config_dir: impl Into<PathBuf>,
        data_local_dir: impl Into<PathBuf>,
        cache_dir: impl Into<PathBuf>,
    ) -> Result<Self> {
        let inner = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or_else(|| eyre!("failed to get project directories"))?;

        Ok(Self {
            config_dir: Some(config_dir.into()),
            data_local_dir: Some(data_local_dir.into()),
            cache_dir: Some(cache_dir.into()),
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

    pub(crate) fn cache_dir(&self) -> &Path {
        self.cache_dir
            .as_deref()
            .unwrap_or_else(|| self.inner.cache_dir())
    }
}
