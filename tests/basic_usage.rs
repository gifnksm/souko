use std::path::PathBuf;

use assert_cmd::prelude::*;
use assert_fs::{TempDir, fixture::ChildPath, prelude::*};
use predicates::prelude::*;

mod common;

fn data_local_dir_path(home_dir: &std::path::Path) -> PathBuf {
    if cfg!(target_os = "linux") {
        return home_dir.join(".local/share/souko");
    }
    if cfg!(target_os = "macos") {
        return home_dir.join("Library/Application Support/souko");
    }
    if cfg!(target_os = "windows") {
        return home_dir.join(r"AppData\Local\souko\data");
    }
    panic!("unsupported platform");
}

fn data_local_dir(home_dir: &impl PathChild) -> ChildPath {
    home_dir.child(data_local_dir_path(std::path::Path::new("")).as_path())
}

#[test]
fn show_help_message() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn clone_and_list() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    data_local_dir(&home)
        .child("root/github.com/gifnksm/souko/.git")
        .assert(predicate::path::is_dir());

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            dunce::canonicalize(
                data_local_dir(&home)
                    .child("root/github.com/gifnksm/souko")
                    .path()
            )
            .unwrap()
            .display()
        ));
}

#[test]
fn relative_integration_test_home_is_absolutized() {
    let workspace = TempDir::new().unwrap();
    let relative_home = PathBuf::from("relative-home");

    common::souko_cmd(&workspace)
        .current_dir(workspace.path())
        .env("HOME", &relative_home)
        .env("SOUKO_INTEGRATION_TEST_HOME", &relative_home)
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    let absolute_home = workspace.path().join(&relative_home);
    assert!(
        data_local_dir_path(&absolute_home)
            .join("root/github.com/gifnksm/souko/.git")
            .is_dir()
    );

    common::souko_cmd(&workspace)
        .current_dir(workspace.path())
        .env("HOME", &relative_home)
        .env("SOUKO_INTEGRATION_TEST_HOME", &relative_home)
        .args(["list"])
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            dunce::canonicalize(
                data_local_dir_path(&absolute_home)
                    .join("root/github.com/gifnksm/souko")
                    .as_path()
            )
            .unwrap()
            .display()
        ));
}
