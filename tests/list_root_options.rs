use std::path::{Path, PathBuf};

use assert_cmd::prelude::*;
use assert_fs::{TempDir, fixture::ChildPath, prelude::*};
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

fn config_dir(home_dir: &impl PathChild) -> ChildPath {
    if cfg!(target_os = "linux") {
        return home_dir.child(".config/souko");
    }
    if cfg!(target_os = "macos") {
        return home_dir.child("Library/Application Support/souko");
    }
    if cfg!(target_os = "windows") {
        return home_dir.child(r"AppData\Roaming\souko\config");
    }
    panic!("unsupported platform");
}

fn write_config(home: &TempDir, body: &str) {
    let config_dir = config_dir(home);
    config_dir.create_dir_all().unwrap();
    config_dir.child("config.toml").write_str(body).unwrap();
}

fn canonical(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap()
}

#[test]
fn list_excludes_bare_repo_by_default_and_includes_it_when_enabled() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    let bare_repo = data_local_dir(&home).child("root/bare.git");
    bare_repo.create_dir_all().unwrap();
    git2::Repository::init_bare(bare_repo.path()).unwrap();

    let default_repo = canonical(
        data_local_dir(&home)
            .child("root/github.com/gifnksm/souko")
            .path(),
    );
    let bare_repo = canonical(bare_repo.path());

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(default_repo.display().to_string()))
        .stdout(predicate::str::contains(bare_repo.display().to_string()).not());

    write_config(
        &home,
        r#"
[[root]]
name = "default"
include_bare_repo = true
"#,
    );

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(default_repo.display().to_string()))
        .stdout(predicate::str::contains(bare_repo.display().to_string()));
}

#[test]
fn list_excludes_hidden_dirs_by_default_and_includes_them_when_enabled() {
    let home = TempDir::new().unwrap();

    let hidden_repo = data_local_dir(&home).child("root/.hidden/repo");
    hidden_repo.create_dir_all().unwrap();
    git2::Repository::init(hidden_repo.path()).unwrap();

    let hidden_repo = canonical(hidden_repo.path());

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(hidden_repo.display().to_string()).not());

    write_config(
        &home,
        r#"
[[root]]
name = "default"
visit_hidden_dirs = true
"#,
    );

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(hidden_repo.display().to_string()));
}

#[test]
fn list_does_not_visit_repo_subdirs_by_default_and_visits_them_when_enabled() {
    let home = TempDir::new().unwrap();

    let parent_repo = data_local_dir(&home).child("root/parent");
    parent_repo.create_dir_all().unwrap();
    git2::Repository::init(parent_repo.path()).unwrap();

    let child_repo = parent_repo.child("child");
    child_repo.create_dir_all().unwrap();
    git2::Repository::init(child_repo.path()).unwrap();

    let parent_repo = canonical(parent_repo.path());
    let child_repo = canonical(child_repo.path());

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(parent_repo.display().to_string()))
        .stdout(predicate::str::contains(child_repo.display().to_string()).not());

    write_config(
        &home,
        r#"
[[root]]
name = "default"
visit_repo_subdirs = true
"#,
    );

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(parent_repo.display().to_string()))
        .stdout(predicate::str::contains(child_repo.display().to_string()));
}

#[test]
fn relative_root_path_in_config_resolves_against_config_file_dir() {
    let home = TempDir::new().unwrap();

    // Create a repository under the config directory using a relative path
    let config_d = config_dir(&home);
    config_d.create_dir_all().unwrap();

    let repo_dir = config_d.child("myrepos/github.com/example/project");
    repo_dir.create_dir_all().unwrap();
    git2::Repository::init(repo_dir.path()).unwrap();

    // Write config with a relative root path — it should resolve against the config file's dir
    config_d.child("config.toml").write_str(
        r#"
[[root]]
name = "default"
path = "myrepos"
"#,
    ).unwrap();

    let expected_repo = canonical(repo_dir.path());

    common::souko_cmd(home.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(expected_repo.display().to_string()));
}

#[test]
fn relative_repo_cache_path_from_cli_resolves_against_cwd() {
    let home = TempDir::new().unwrap();
    let workdir = TempDir::new().unwrap();

    // Clone a repo first so there is something to list.
    common::souko_cmd(home.path())
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success();

    let expected_repo = canonical(
        data_local_dir(&home)
            .child("root/github.com/gifnksm/souko")
            .path(),
    );

    // Run list with a relative --repo-cache path from workdir; the cache file should be
    // created in the CWD (workdir), not in the default cache location.
    common::souko_cmd(home.path())
        .current_dir(workdir.path())
        .args(["--repo-cache", "repos.json", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(expected_repo.display().to_string()));

    // Verify the cache was written to the CWD, not somewhere else.
    workdir
        .child("repos.json")
        .assert(predicate::path::is_file());
}
