# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.3.0] - 2021-03-26
### Changed
- Renamed `NFDError` to `NfdError`.
- Updated the underlying C library to the latest devel branch.

### Fixed
- Fixed a memory leak, other than multi-file dialogs, all paths were being leaked, oops!

## [0.2.3] - 2020-10-22
### Added
- [PR#15](https://github.com/EmbarkStudios/nfd2/pull/15) added support for FreeBSD. Thanks [@Erk-](https://github.com/Erk-)!

## [0.2.2] - 2020-09-27
### Added
- `Response` now implements `Clone` and `PartialEq`, thanks [@virtualritz](https://github.com/virtualritz)!

## [0.2.1] - 2020-05-11
### Changed
- [PR#9](https://github.com/EmbarkStudios/nfd2/pull/9) implemented `std::error::Error` for `NFDError`

## [0.2.0] - 2020-04-05
### Changed
- [PR#5](https://github.com/EmbarkStudios/nfd2/pull/5) changed the API to take `Path` inputs and give `PathBuf` outputs for all filesystem paths.

## [0.1.1] - 2020-03-19
### Fixed
- Fixed up cargo metadata

## [0.1.0] - 2020-03-19
### Added
- Initial add of nfd2, forked from [nfd-rs](https://github.com/saurvs/nfd-rs)

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/nfd2/compare/0.3.0...HEAD
[0.3.0]: https://github.com/EmbarkStudios/nfd2/compare/0.2.3...0.3.0
[0.2.3]: https://github.com/EmbarkStudios/nfd2/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/EmbarkStudios/nfd2/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/EmbarkStudios/nfd2/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/nfd2/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/EmbarkStudios/nfd2/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/EmbarkStudios/nfd2/releases/tag/0.1.0
