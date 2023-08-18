use std::{path::PathBuf, sync::Arc};

use crate::domain::{
    model::path_like::PathLike,
    repository::edit_dir::{EditDir, EnsureDirExistError},
};

#[derive(Debug)]
pub(in super::super) struct Workdir {
    edit_dir: Arc<dyn EditDir>,
    created_dirs: Vec<PathBuf>,
    erase_leaf_content: bool,
}

impl Workdir {
    pub(in super::super) fn create(
        edit_dir: Arc<dyn EditDir>,
        path: &dyn PathLike,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut workdir = Self {
            edit_dir: Arc::clone(&edit_dir),
            created_dirs: vec![],
            erase_leaf_content: false,
        };
        let path = path.as_real_path();
        let leaf_path = path;
        let mut to_create = vec![leaf_path];
        while let Some(path) = to_create.last().copied() {
            match edit_dir.ensure_dir_exist(path) {
                Ok(created_by_call) => {
                    if created_by_call {
                        // register the created dir only if it is created by this call.
                        workdir.created_dirs.push(path.to_owned());
                        if path == leaf_path {
                            // erase content only if the leaf dir is created by this call.
                            workdir.erase_leaf_content = true;
                        }
                    }
                    to_create.pop();
                    continue;
                }
                Err(EnsureDirExistError::ParentNotExist { .. }) => {
                    if let Some(parent) = path.parent() {
                        to_create.push(parent);
                        continue;
                    }
                    unreachable!()
                }
                Err(err) => bail!(err),
            }
        }
        Ok(workdir)
    }

    pub(in super::super) fn persist(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.erase_leaf_content = false;
        self.created_dirs.clear();
        Ok(())
    }
}

impl Drop for Workdir {
    fn drop(&mut self) {
        if self.erase_leaf_content {
            if let Some(leaf_path) = self.created_dirs.last() {
                let res = self.edit_dir.ensure_dir_empty(leaf_path);
                if res.is_err() {
                    return;
                }
            }
        }

        while let Some(last) = self.created_dirs.pop() {
            let res = self.edit_dir.ensure_dir_removed(&last);
            if res.is_err() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use assert_fs::{prelude::*, TempDir};

    use super::*;
    use crate::infrastructure;

    #[test]
    fn persist() {
        let repository = infrastructure::repository();
        let edit_dir = repository.edit_dir; // must be `FsEditDir`

        let test_dir = TempDir::new().unwrap();
        test_dir.child("dir1/").create_dir_all().unwrap();
        let workdir_path = test_dir.child("dir1/a/b/c");
        let mut workdir = Workdir::create(Arc::clone(&edit_dir), &workdir_path.path()).unwrap();
        assert!(workdir_path.is_dir());
        workdir_path.child("file").touch().unwrap();
        // `persist()` prevents workdir and its content from being removed on drop.
        workdir.persist().unwrap();
        drop(workdir);
        // check directory and file still exist.
        assert!(workdir_path.is_dir());
        assert!(workdir_path.child("file").is_file());

        // Ensure `test_dir` and its contents are deleted
        test_dir.close().unwrap();
    }

    #[test]
    fn remove_on_drop() {
        let repository = infrastructure::repository();
        let edit_dir = repository.edit_dir; // must be `FsEditDir`

        let test_dir = TempDir::new().unwrap();

        // when some directories created by `Workdir` are removed on drop
        test_dir.child("dir1/").create_dir_all().unwrap();
        let workdir_path = test_dir.child("dir1/a/b/c");
        let workdir = Workdir::create(Arc::clone(&edit_dir), &workdir_path.path()).unwrap();
        assert!(workdir_path.is_dir());
        workdir_path.child("file").touch().unwrap();
        // drop removes the workdir and its content.
        drop(workdir);
        assert!(!workdir_path.try_exists().unwrap());
        // dir1 exists and is empty.
        assert!(test_dir.child("dir1").try_exists().unwrap());
        assert!(test_dir.child("dir1").read_dir().unwrap().next().is_none());

        // when no directory is created by `Workdir`, nothing is removed on drop.
        test_dir.child("dir2/a/b/c").create_dir_all().unwrap();
        let workdir_path = test_dir.child("dir2/a/b/c");
        let workdir = Workdir::create(Arc::clone(&edit_dir), &workdir_path.path()).unwrap();
        assert!(workdir_path.is_dir());
        workdir_path.child("file").touch().unwrap();
        // drop removes nothing
        drop(workdir);
        assert!(workdir_path.is_dir());
        assert!(workdir_path.child("file").is_file());

        // Ensure `test_dir` and its contents are deleted
        test_dir.close().unwrap();
    }
}
