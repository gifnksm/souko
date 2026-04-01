use std::{path::Path, process::Command};

use assert_cmd::cargo::CommandCargoExt;

pub fn souko_cmd(home_dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.envs([
        ("HOME", home_dir.as_os_str()),
        ("SOUKO_INTEGRATION_TEST", "true".as_ref()),
    ]);
    cmd
}
