# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0] - 2026-04-03

### Added

* `souko list --template <FORMAT>`: new template-based output format for customizing list output ([#643](https://github.com/gifnksm/souko/pull/643), [#644](https://github.com/gifnksm/souko/pull/644))
  * Useful for integrating with picker tools such as fzf and skim
  * Available template variables include `root_name`, `repo_relative_path`, `repo_canonical_path`, etc.
  * Unknown variable names in templates are detected and reported as errors when running `souko list` (during `--template` argument validation)
* Added shell integration documentation and a zsh plugin for selecting a repository with `sk` or `fzf` and changing to it from the current shell ([#658](https://github.com/gifnksm/souko/pull/658))
  * Documented fuzzy-finder-based repository navigation in the README
  * Added `souko.plugin.zsh` and `souko-cd-widget` with configurable selector and key binding options

### Fixed

* `query.scheme_alias` and `query.custom_scheme` fields in `config.toml` can now be omitted without causing a parse error ([#645](https://github.com/gifnksm/souko/pull/645))

### Changed

* Rust edition bumped to 2024 ([#650](https://github.com/gifnksm/souko/pull/650))
* Error message wording and contribution guidelines were updated to clarify the project's error display policy ([#651](https://github.com/gifnksm/souko/pull/651))
* Added a `just ci-lint` workflow for lightweight local verification and updated contributor guidance accordingly ([#652](https://github.com/gifnksm/souko/pull/652))
* Minimum Supported Rust Version (MSRV) bumped to 1.88.0 (was 1.78.0)
* Updated dependencies

## [0.2.2] - 2024-12-27

### Changed

* Minimum Supported Rust Version (MSRV) bumped to 1.78.0 (was 1.74.0) ([#491](https://github.com/gifnksm/souko/pull/491))
* Updated dependencies

## [0.2.1] - 2024-09-11

### Fixed

* Output paths on Windows no longer include the UNC prefix (`\\?\`) ([#462](https://github.com/gifnksm/souko/pull/462))

### Changed

* Minimum Supported Rust Version (MSRV) bumped to 1.74.0 (was 1.70.0) ([#463](https://github.com/gifnksm/souko/pull/463))
* Updated dependencies

## [0.2.0] - 2023-09-17

### Added

* `vendored-libgit2` and `vendored-openssl` feature flags, which allow building without system libraries ([#297](https://github.com/gifnksm/souko/pull/297))

### Changed

* **(Breaking)** System `libgit2` and `openssl` are now used by default instead of vendored copies ([#308](https://github.com/gifnksm/souko/pull/308))
  * To build without system dependencies, pass `--features vendored-libgit2,vendored-openssl`

## [0.1.2] - 2023-09-03

### Added

* Repository cache ([#295](https://github.com/gifnksm/souko/pull/295))

## [0.1.1] - 2023-09-02

### Fixed

* `query` and `root` sections in `config.toml` are now optional ([#293](https://github.com/gifnksm/souko/pull/293))

## [0.1.0] - 2023-09-02

* First release

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/souko/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/gifnksm/souko/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/gifnksm/souko/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/gifnksm/souko/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/gifnksm/souko/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/gifnksm/souko/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/gifnksm/souko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/souko/commits/v0.1.0
