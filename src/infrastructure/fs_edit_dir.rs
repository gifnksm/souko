use std::{fs, io, path::Path};

use remove_dir_all::remove_dir_contents;

use crate::domain::repository::edit_dir::{EditDir, EnsureDirExistError};

#[derive(Debug)]
pub(super) struct FsEditDir {}

impl FsEditDir {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl EditDir for FsEditDir {
    fn ensure_dir_exist(&self, path: &Path) -> Result<bool, EnsureDirExistError> {
        match fs::create_dir(path) {
            Ok(()) => Ok(true),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists && path.is_dir() => Ok(false),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                Err(EnsureDirExistError::ParentNotExist {
                    source: Box::new(err),
                })
            }
            Err(err) => Err(EnsureDirExistError::Other(Box::new(err))),
        }
    }

    fn ensure_dir_removed(
        &self,
        path: &Path,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let removed = match fs::remove_dir(path) {
            Ok(()) => true,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    if parent.is_dir() {
                        return Ok(false);
                    }
                }
                bail!(err)
            }
            Err(err) => bail!(err),
        };
        Ok(removed)
    }

    fn ensure_dir_empty(
        &self,
        path: &Path,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut dirs = path.read_dir()?;
        let is_empty = dirs.next().is_none();
        if is_empty {
            return Ok(false);
        }
        drop(dirs);

        remove_dir_contents(path)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::{prelude::*, TempDir};

    use super::*;

    trait SetReadonly {
        fn set_readonly(&self, readonly: bool) -> io::Result<()>;
    }

    impl SetReadonly for assert_fs::fixture::ChildPath {
        fn set_readonly(&self, readonly: bool) -> io::Result<()> {
            let mut perms = fs::metadata(self)?.permissions();
            perms.set_readonly(readonly);
            fs::set_permissions(self, perms)
        }
    }

    #[test]
    fn ensure_dir_exist() {
        use EnsureDirExistError as Error;

        let test_dir = TempDir::new().unwrap();

        let normal_dir = test_dir.child("normal_dir");
        normal_dir.create_dir_all().unwrap();

        let non_empty_dir = test_dir.child("non_empty_dir");
        non_empty_dir.create_dir_all().unwrap();
        non_empty_dir.child("file").touch().unwrap();

        let not_a_directory = test_dir.child("not_a_directory");
        not_a_directory.touch().unwrap();

        let readonly_dir = test_dir.child("readonly_dir");
        readonly_dir.create_dir_all().unwrap();
        readonly_dir.set_readonly(true).unwrap();

        let edit_dir = FsEditDir::new();

        // Test postcondition
        let call = |path: &Path| {
            let res = edit_dir.ensure_dir_exist(path);
            if res.is_ok() {
                assert!(path.is_dir());
            }
            res
        };

        // Tests the properties described in the doc comment of `EditDir::ensure_dir_exist`

        // Returns `true` if if the directory was created by this function call
        assert!(call(&normal_dir.child("newdir1")).unwrap());
        assert!(call(&normal_dir.child("newdir2")).unwrap());

        // Returns `false` if the directory already exists
        assert!(!call(&normal_dir.child("newdir1")).unwrap());
        assert!(!call(&normal_dir.child("newdir2")).unwrap());
        assert!(!call(&non_empty_dir).unwrap());

        // Returns `Err` if a non-directory file already exists at the `path`
        assert!(matches!(call(&not_a_directory), Err(Error::Other { .. })));

        // Returns `Err` if the parent of the given `path` does not exist
        assert!(matches!(
            call(&normal_dir.child("parent/child")),
            Err(Error::ParentNotExist { .. })
        ));

        // Returns `Err` if the user does not have permission to create a directory
        #[cfg(unix)]
        assert!(matches!(
            call(&readonly_dir.child("newdir")),
            Err(Error::Other { .. })
        ));

        // Ensure `test_dir` and its contents are deleted
        readonly_dir.set_readonly(false).unwrap();
        test_dir.close().unwrap();
    }

    #[test]
    fn ensure_dir_removed() {
        let test_dir = TempDir::new().unwrap();

        let normal_dir1 = test_dir.child("normal_dir/newdir1");
        normal_dir1.create_dir_all().unwrap();
        let normal_dir2 = test_dir.child("normal_dir/newdir2");
        normal_dir2.create_dir_all().unwrap();

        let non_empty_dir = test_dir.child("non_empty_dir");
        non_empty_dir.create_dir_all().unwrap();
        non_empty_dir.child("file").touch().unwrap();

        let not_a_directory = test_dir.child("not_a_directory");
        not_a_directory.touch().unwrap();

        let readonly_dir = test_dir.child("readonly_dir");
        let undeletable_dir = readonly_dir.child("undeletable_dir");
        readonly_dir.create_dir_all().unwrap();
        undeletable_dir.create_dir_all().unwrap();
        readonly_dir.set_readonly(true).unwrap();

        let edit_dir = FsEditDir::new();

        // Test postcondition
        let call = |path: &Path| {
            let res = edit_dir.ensure_dir_removed(path);
            if res.is_ok() {
                assert!(path.parent().unwrap().try_exists().unwrap());
                assert!(!path.try_exists().unwrap());
            }
            res
        };

        // Tests the properties described in the doc comment of `EditDir::ensure_dir_removed`

        // Returns `true` if the directory was removed by this function call
        assert!(call(&normal_dir1).unwrap());
        assert!(call(&normal_dir2).unwrap());

        // Returns `false` if the directory does not already exist
        assert!(!call(&normal_dir1).unwrap());
        assert!(!call(&normal_dir2).unwrap());

        // Returns `Err` if the directory is not empty
        assert!(matches!(call(&non_empty_dir), Err(_)));

        // Returns `Err` if the given `path` is not a directory
        assert!(matches!(call(&not_a_directory), Err(_)));

        // Returns `Err` if the parent of the given `path` does not exist
        assert!(matches!(call(&test_dir.child("parent/child")), Err(_)));

        // Returns `Err` if the user does not have permission to remove the directory
        #[cfg(unix)]
        assert!(matches!(call(&undeletable_dir), Err(_)));

        // Ensure `test_dir` and its contents are deleted
        readonly_dir.set_readonly(false).unwrap();
        test_dir.close().unwrap();
    }

    #[test]
    fn ensure_dir_empty() {
        let test_dir = TempDir::new().unwrap();

        let normal_dir1 = test_dir.child("normal_dir/newdir1");
        normal_dir1.create_dir_all().unwrap();
        normal_dir1.child("file").touch().unwrap();
        let normal_dir2 = test_dir.child("normal_dir/newdir2");
        normal_dir2.create_dir_all().unwrap();
        normal_dir2.child("file").touch().unwrap();

        let empty_dir = test_dir.child("empty_dir");
        empty_dir.create_dir_all().unwrap();

        let not_a_directory = test_dir.child("not_a_directory");
        not_a_directory.touch().unwrap();
        let not_exist = test_dir.child("not_exist");

        let readonly_dir = test_dir.child("readonly_dir");
        readonly_dir.create_dir_all().unwrap();
        readonly_dir.child("file").touch().unwrap();
        readonly_dir.set_readonly(true).unwrap();

        let edit_dir = FsEditDir::new();

        // Test postcondition
        let call = |path: &Path| {
            let res = edit_dir.ensure_dir_empty(path);
            if res.is_ok() {
                assert!(path.is_dir());
                assert!(path.read_dir().unwrap().next().is_none());
            }
            res
        };

        // Tests the properties described in the doc comment of `EditDir::ensure_dir_removed`

        // Returns `true` if the contents of the directory were removed by this function call
        assert!(call(&normal_dir1).unwrap());
        assert!(call(&normal_dir2).unwrap());

        // Returns `false` if the directory is already empty
        assert!(!call(&normal_dir1).unwrap());
        assert!(!call(&normal_dir2).unwrap());
        assert!(!call(&empty_dir).unwrap());

        // Returns `Err` if `path` is not a directory or does not exist
        assert!(matches!(call(&not_a_directory), Err(_)));
        assert!(matches!(call(&not_exist), Err(_)));

        // Returns `Err` if the user does not have permission to remove the contents of the directory
        #[cfg(unix)]
        assert!(matches!(call(&readonly_dir), Err(_)));

        // Ensure `test_dir` and its contents are deleted
        readonly_dir.set_readonly(false).unwrap();
        test_dir.close().unwrap();
    }
}
