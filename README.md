<!-- cargo-sync-rdme title [[ -->
# souko
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
[![Maintenance: actively-developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg?style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/souko.svg?style=flat-square)](#license)
[![crates.io](https://img.shields.io/crates/v/souko.svg?logo=rust&style=flat-square)](https://crates.io/crates/souko)
[![docs.rs](https://img.shields.io/docsrs/souko.svg?logo=docs.rs&style=flat-square)](https://docs.rs/souko)
[![Rust: ^1.88.0](https://img.shields.io/badge/rust-^1.88.0-93450a.svg?logo=rust&style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![GitHub Actions: CI](https://img.shields.io/github/actions/workflow/status/gifnksm/souko/ci.yml.svg?label=CI&logo=github&style=flat-square)](https://github.com/gifnksm/souko/actions/workflows/ci.yml)
[![Codecov](https://img.shields.io/codecov/c/github/gifnksm/souko.svg?label=codecov&logo=codecov&style=flat-square)](https://codecov.io/gh/gifnksm/souko)
<!-- cargo-sync-rdme ]] -->

Souko is a simple command line utility that provides an easy way to organize clones of remote git repositories.

VS Code extension [souko-vscode] is also available.

[souko-vscode]: https://marketplace.visualstudio.com/items?itemName=gifnksm.souko

When you clone a remote repository with souko, souko creates a directory under a specific root directory (`~/.local/share/souko/root` by default) containing the hostname and path of the remote repository's URL.

```console
$ souko clone https://github.com/gifnksm/souko
# => creates local clone at ~/.local/share/souko/root/github.com/gifnksm/souko
```

You can also list all repositories that have been cloned.

```console
$ souko list
# => list of absolute paths of all repositories cloned with souko
```

For picker tools such as fzf/skim, you can customize each output line with `--template`.

```console
$ souko list --template $'{root_name}\t{repo_relative_path}\t{repo_canonical_path}'
# => default    github.com/gifnksm/souko    /home/you/.local/share/souko/root/github.com/gifnksm/souko
```

`--template` uses souko's template variables (no extra escape-sequence processing is done by souko itself; quoting/escaping is handled by your shell).

Terminology:

- `root`: one `[[root]]` entry in config (for example, `default` or `repos`)
- `repo`: each Git repository found under a root

Available variables:

- `{root_name}`
- `{root_display_path}`
- `{root_real_path}`
- `{root_canonical_path}`
- `{repo_relative_path}`
- `{repo_display_path}`
- `{repo_real_path}`
- `{repo_canonical_path}`

Path variable semantics:

- `relative_path`: path relative to the selected root (for repositories, this is `{repo_relative_path}`)
- `display_path`: user-facing path representation (may include `~` and may not be absolute)
- `real_path`: absolute path before canonicalization (symlinks are not resolved)
- `canonical_path`: canonical absolute path (symlinks resolved)

By combining souko, fuzzy finder, and shell functions, you can easily jump between repositories.

Example with skim (`sk`):

```console
$ repo_dir="$(
    souko list --template $'{root_name} {repo_relative_path}\t{repo_canonical_path}' |
      sk --delimiter $'\t' --with-nth 1 --nth 1 |
      cut -f2
  )"
$ printf '%s\n' "$repo_dir"
```

Example with fzf:

```console
$ repo_dir="$(
    souko list --template $'{root_name} {repo_relative_path}\t{repo_canonical_path}' |
      fzf --delimiter=$'\t' --with-nth=1 |
      cut -f2
  )"
$ printf '%s\n' "$repo_dir"
```

## Configuration

Configuration is done via a TOML file located at `~/.config/souko/config.toml` by default.

```toml
[[root]]
name = "default"
path = "~/.local/share/souko/root"

[[root]]
name = "repos"
path = "~/repos"

[query]
default_scheme = "github"

[query.scheme_alias]
gh = "github"
gl = "gitlab"

[query.custom_scheme]
github = "https://github.com/{path}.git"
gitlab = "https://gitlab.com/{path}.git"
```

## Installation

There are multiple ways to install souko.
Choose any one of the methods below that best suits your needs.

### Distribution packages

Following packages are available:

- Arch Linux (AUR): [souko](https://aur.archlinux.org/packages/souko/) or [souko-bin](https://aur.archlinux.org/packages/souko-bin/)

### Pre-built binaries

Executable binaries are available for download on the [GitHub Release page].

You can also install the binary with [`cargo-binstall`] command.

```console
# Install pre-built binary
$ cargo binstall souko
```

[GitHub Release page]: https://github.com/gifnksm/souko/releases/
[`cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall

### Build from source using Rust

To build souko executable from the source, you must have the Rust toolchain installed.
To install the rust toolchain, follow [this guide](https://www.rust-lang.org/tools/install).

Once you have installed Rust, the following command can be used to build and install souko:

```console
# Install released version
$ cargo install souko

# Install latest version
$ cargo install --git https://github.com/gifnksm/souko.git souko
```

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.88.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.

## License

This project is licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
