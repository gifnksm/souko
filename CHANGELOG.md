# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.2] - 2024-12-27

## [0.2.1] - 2024-09-11

## [0.2.0] - 2023-09-17

### Changed

* (breaking change) Use system `libgit2` and `libopenssl` by default
  * If you want to use `souko` command without system dependencies, please build `souko` with`--features vendored-libgit2,vendored-libopenssl` flag.

## [0.1.2] - 2023-09-03

### Added

* Repository cache

## [0.1.1] - 2023-09-02

### Fixed

* config.toml: `query` and `root` are now optional.

## [0.1.0] - 2023-09-02

* First release

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/souko/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/gifnksm/souko/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/gifnksm/souko/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/gifnksm/souko/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/gifnksm/souko/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/gifnksm/souko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/souko/commits/v0.1.0
