use std::{fmt::Debug, path::Path};

#[derive(Debug, thiserror::Error)]
pub(crate) enum EnsureDirExistError {
    #[error("parent directory not exist")]
    ParentNotExist {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub(crate) trait EditDir: Debug {
    /// Creates a directory if it does not exist.
    ///
    /// Returns `true` if the directory was created by this function call, or
    /// `false` if the directory already exists.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * A non-directory file already exists at the `path`.
    /// * The parent of the given `path` does not exist.
    /// * The user does not have permission to create a directory at the given `path`.
    fn ensure_dir_exist(&self, path: &Path) -> Result<bool, EnsureDirExistError>;

    /// Removes the empty directory if it exists.
    ///
    /// Returns `true` if the directory was removed by this function call, or
    /// `false` if the directory does not already exist.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * The directory is not empty.
    /// * The given `path` is not a directory.
    /// * The parent of the given `path` does not exist.
    /// * The user does not have permission to remove the directory at the given `path`.
    fn ensure_dir_removed(
        &self,
        path: &Path,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>;

    /// Removes the contents of the directory if they exist.
    ///
    /// Returns `true` if the contents of the directory were removed by this function
    /// call, or `false` if the directory is already empty.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * The given `path` is not a directory or does not exist.
    /// * The user does not have permission to remove the contents of the directory at the
    ///   given `path`.
    fn ensure_dir_empty(
        &self,
        path: &Path,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assert object safety for trait object.
    const _: Option<&dyn EditDir> = None;
}
