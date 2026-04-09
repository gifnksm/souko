use std::{path::PathBuf, sync::Arc};

use crate::domain::{
    model::path_like::PathLike,
    port::dir_editor::{DirEditor, EnsureDirExistError},
};

#[derive(Debug)]
pub(in super::super) struct WorkdirGuard {
    dir_editor: Arc<dyn DirEditor>,
    created_dirs: Vec<PathBuf>,
    erase_leaf_content: bool,
}

impl WorkdirGuard {
    pub(in super::super) fn create(
        dir_editor: Arc<dyn DirEditor>,
        path: &dyn PathLike,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut workdir = Self {
            dir_editor: Arc::clone(&dir_editor),
            created_dirs: vec![],
            erase_leaf_content: false,
        };
        let path = path.as_real_path();
        let leaf_path = path;
        let mut to_create = vec![leaf_path];
        while let Some(path) = to_create.last().copied() {
            match dir_editor.ensure_dir_exist(path) {
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
                Err(err) => return Err(err.into()),
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

impl Drop for WorkdirGuard {
    fn drop(&mut self) {
        if self.erase_leaf_content
            && let Some(leaf_path) = self.created_dirs.last()
        {
            let res = self.dir_editor.ensure_dir_empty(leaf_path);
            if res.is_err() {
                return;
            }
        }

        while let Some(last) = self.created_dirs.pop() {
            let res = self.dir_editor.ensure_dir_removed(&last);
            if res.is_err() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use assert_fs::{TempDir, prelude::*};

    use super::*;
    use crate::infrastructure;

    #[test]
    fn persist() {
        let ports = infrastructure::ports();
        let dir_editor = ports.dir_editor; // must be `FsDirEditor`

        let test_dir = TempDir::new().unwrap();
        test_dir.child("dir1/").create_dir_all().unwrap();
        let workdir_path = test_dir.child("dir1/a/b/c");
        let mut workdir =
            WorkdirGuard::create(Arc::clone(&dir_editor), &workdir_path.path()).unwrap();
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
        let ports = infrastructure::ports();
        let dir_editor = ports.dir_editor; // must be `FsDirEditor`

        let test_dir = TempDir::new().unwrap();

        // when some directories created by `Workdir` are removed on drop
        test_dir.child("dir1/").create_dir_all().unwrap();
        let workdir_path = test_dir.child("dir1/a/b/c");
        let workdir = WorkdirGuard::create(Arc::clone(&dir_editor), &workdir_path.path()).unwrap();
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
        let workdir = WorkdirGuard::create(Arc::clone(&dir_editor), &workdir_path.path()).unwrap();
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
