# souko

[![maintenance status: actively-developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license: MIT OR APACHE-2.0](https://img.shields.io/crates/l/souko.svg)](#license)
[![crates.io](https://img.shields.io/crates/v/souko.svg)](https://crates.io/crates/souko)
[![docs.rs](https://docs.rs/souko/badge.svg)](https://docs.rs/souko/)
[![rust 1.60.0+ badge](https://img.shields.io/badge/rust-1.60.0+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/gifnksm/souko/actions/workflows/ci.yml/badge.svg)](https://github.com/gifnksm/souko/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/gifnksm/souko/graph/badge.svg)](https://codecov.io/gh/gifnksm/souko)

Souko is a simple command line utility that provides an easy way to organize clones of remote git repositories.

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

You can also import a cloned local repository into souko.
Imported repositories are also listed by the list command.

```console
$ souko import -r ~/repos
# => import all repositories under ~/repos into souko
```

By combining souko, fuzzy finder, and shell functions, you can easily jump between repositories (TODO: add shell script example).

## Installation

There are multiple ways to install souko.
Choose any one of the methods below that best suits your needs.

### Pre-built binaries

Executable binaries are available for download on the [GitHub Release page].

[GitHub Release page]: https://github.com/gifnksm/souko/releases/

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

The minimum supported Rust version is **Rust 1.60.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.

## License

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
