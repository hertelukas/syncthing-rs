# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.1](https://github.com/hertelukas/syncthing-rs/compare/v0.1.0-alpha.0...v0.1.0-alpha.1) - 2025-05-09

### Added

- add initial #[derive(New)] macro crate scaffolding
- default endpoints

### Other

- add badges to cargo.io
- release v0.1.0-alpha.0

## [0.1.0-alpha.0](https://github.com/hertelukas/syncthing-rs/releases/tag/v0.1.0-alpha.0) - 2025-05-08

### Other

- Setup release-plz
- Fixed typo
- Package setup
- Create ci.yml
- Create LICENSE-APACHE
- Create LICENSE-MIT
- used cargo clippy to fix issues
- Using broadcast instead of mpsc
- Debug for client
- cargo fmt
- Cluster endpoints
- cluster/pending types
- Functions to deal with single folders/devices
- Endpoints for health and getting device id
- Added logging
- Testing events
- Test for ping endpoint
- Function to load config
- Make lines pub
- Complete config types
- DeviceConfiguration complete
- Configuration complete up to FolderConfiguration
- Basic project structure
- Initial commit
