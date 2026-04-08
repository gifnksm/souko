use std::{path::Path, process::Command};

use assert_cmd::cargo::CommandCargoExt;

pub fn souko_cmd(home_dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("souko").unwrap();
    cmd.envs([
        // Keep HOME aligned with the temporary test home so home-based path resolution
        // outside ProjectDirs (for example, tilde expansion via BaseDirs) stays
        // deterministic in integration tests. See also issue #673.
        ("HOME", home_dir.as_os_str()),
        // Override ProjectDirs explicitly so integration tests do not depend on the
        // host environment's directories/XDG settings.
        ("SOUKO_INTEGRATION_TEST_HOME", home_dir.as_os_str()),
    ]);
    cmd
}
