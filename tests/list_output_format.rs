use std::path::Path;

use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use serde_json::Value;

mod common;

#[test]
fn list_with_template_output() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    let output = common::souko_cmd(home.path())
        .args([
            "list",
            "--template",
            "{root_name}\t{repo_relative_path}\t{repo_canonical_path}",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output = String::from_utf8(output).unwrap();
    let line = output.trim_end_matches('\n');
    let mut fields = line.split('\t');

    let root_name = fields.next().unwrap();
    let repo_relative_path = fields.next().unwrap();
    let repo_canonical_path = fields.next().unwrap();

    assert_eq!(root_name, "default");
    // Compare as `Path` values so the assertion is OS-independent:
    // Windows uses `\` while Unix-like systems use `/` as separators.
    assert_eq!(
        Path::new(repo_relative_path),
        Path::new("github.com/gifnksm/souko")
    );
    assert!(Path::new(repo_canonical_path).is_absolute());
    assert!(fields.next().is_none());
}

#[test]
fn list_template_and_json_are_mutually_exclusive() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["list", "--json", "--template", "{root_name}"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--json"))
        .stderr(predicate::str::contains("--template"));
}

#[test]
fn list_json_output_is_valid_json_after_template_addition() {
    let home = TempDir::new().unwrap();

    common::souko_cmd(home.path())
        .args(["clone", "gifnksm/souko"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    let output = common::souko_cmd(home.path())
        .args(["list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: Value = serde_json::from_slice(&output).unwrap();
    assert!(value.get("roots").is_some());
}
