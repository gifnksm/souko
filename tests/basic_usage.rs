use assert_cmd::prelude::*;
use assert_fs::{fixture::ChildPath, prelude::*, TempDir};
use predicates::prelude::*;

mod common;

fn data_local_dir(home_dir: &impl PathChild) -> ChildPath {
    if cfg!(target_os = "linux") {
        return home_dir.child(".local/share/souko");
    }
    if cfg!(target_os = "macos") {
        return home_dir.child("Library/Application Support/souko");
    }
    if cfg!(target_os = "windows") {
        return home_dir.child(r"AppData\Local\souko\data");
    }
    panic!("unsupported platform");
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
