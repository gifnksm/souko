use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;

#[test]
fn show_help_message() {
    let home = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.env("HOME", home.path()).args(["--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
}

#[test]
fn clone_and_list() {
    let home = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.env("HOME", home.path())
        .args(["clone", "gifnksm/souko"]);
    cmd.assert().success().stdout(predicate::str::is_empty());

    home.child(".local/share/souko/root/github.com/gifnksm/souko/.git")
        .assert(predicate::path::is_dir());
    home.child(".local/share/souko/repo_index.json")
        .assert(predicate::path::is_file());

    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.env("HOME", home.path()).args(["list"]);
    cmd.assert().success().stdout(format!(
        "{}/.local/share/souko/root/github.com/gifnksm/souko\n",
        home.path().display()
    ));
}

#[test]
fn import_and_list() {
    let repo = TempDir::new().unwrap();
    git2::Repository::clone("https://github.com/gifnksm/souko.git", repo.path()).unwrap();
    repo.child(".git").assert(predicate::path::is_dir());

    let home = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.env("HOME", home.path())
        .args(["import", repo.path().display().to_string().as_str()]);
    cmd.assert().success().stdout(predicate::str::is_empty());

    home.child(".local/share/souko/repo_index.json")
        .assert(predicate::path::is_file());

    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.env("HOME", home.path()).args(["list"]);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", repo.path().display()));
}
