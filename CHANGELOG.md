# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]


## [0.2.3] - 2016-11-23
### Added
- Custom `u16` colors

### Changed
- Improve documentation


## [0.2.2] - 2016-01-14
### Changed
- Version wildcard in dependency was replaced by more specific version

### Fixed
- Builds with `term 0.4` now


## [0.2.1] - 2015-08-27
### Fixed
- `fmt` traits implementation: forward formatting parameter correctly

## [0.2.0] - 2015-07-10
### Added
- `ToStyle::not_underline()`
- Implementation for all `fmt` formatting traits, not just `Debug` and
  `Display`

### Changed
- Improve documentation
- Reduce size of `Style` from 14 to 4 bytes
- Rename `Color::Normal` to `Color::NotSet`


## [0.1.1] - 2015-04-15
### Added
- `ToStyle::with()`

### Changed
- Improve documentation
- Relax type constraints on `ToStyle`

### Fixed
- Resetting styles works correctly now

## 0.1.0 - 2015-04-14
*Initial release*

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.3.0...HEAD
[0.2.3]: https://github.com/LukasKalbertodt/term-painter/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/LukasKalbertodt/term-painter/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/LukasKalbertodt/term-painter/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/LukasKalbertodt/term-painter/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/LukasKalbertodt/term-painter/compare/0.1.0...0.1.1
