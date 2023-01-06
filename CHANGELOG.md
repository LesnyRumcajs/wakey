# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2023-01-06


### Added

- this changelog :)

### Changed

- refactored the interface and internals a bit. In short, the library should no longer panic on any errors and should return a sane `Result` so it can be handled by the crate user.
- made methods `WolPacket::from_string` and `WolPacket::mac_to_byte` more flexible by making the `data` an `AsRef<str>`.
- `wakey-wake` no longer needs an `-m` or `--mac` flag. The adress is now a positional argument so just call the binary with the appropriate address.
