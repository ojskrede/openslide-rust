# Changelog

This project follows [Semantic Versioning](https://semver.org/).

The changelog format is based on [this template](http://keepachangelog.com/en/1.0.0/).

### Legend
  - `Added` for new features.
  - `Changed` for changes in existing functionality.
  - `Deprecated` for once-stable features removed in upcoming releases.
  - `Removed` for deprecated features removed in this release.
  - `Fixed` for any bug fixes.
  - `Security` to invite users to upgrade in case of vulnerabilities.

## TODO:
  - Test for native bindings.
  - Implement properties structs for more formats.
  - Fix OpenSlide best level suggestions. Example from Aperio:
factor -> best level -> factor for level
16.0 -> 1 -> 4
16.1 -> 2 -> 16

## [0.2.1] -
### Fixed
  - A bug introduced when adhering to clippy caused no properties to be parsed. This is fixed.
### Removed
  - The assets folder was accidentally included in the packaging. This is removed now.

## [0.2.0] - 2018.11.19
### Added
  - Integration tests for convenience api.
  - Guards for size and location of read frame based on slide dimensions.
  - Properties structs (currently only implemented for some formats).
### Changed
  - Substituted primitive types with generic floats and integers.
  - Bumped image crate to 0.20.

## [0.1.0] - 2018.06.22
### Added
  - Basic functionality is in place.
  - Created the openslide rust crate.

## [0.0.9] - 2018.05.29
### Added
  - This is the first commit.
  - Merged existing projects to create this one.
