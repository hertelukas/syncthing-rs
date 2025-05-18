# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.2](https://github.com/hertelukas/syncthing-rs/compare/syncthing-macros-v0.1.0-alpha.1...syncthing-macros-v0.1.0-alpha.2) - 2025-05-18

### Added

- #[derive(ParitalEq)] for all config types
- *(macros)* #[derive(New)] types are clone

## [0.1.0-alpha.1](https://github.com/hertelukas/syncthing-rs/compare/syncthing-macros-v0.1.0-alpha.0...syncthing-macros-v0.1.0-alpha.1) - 2025-05-12

### Added

- *(macros)* allow into new from initial type
- *(macros)* propagate rename_all attribute

### Fixed

- *(macros)* rename required fields too

### Other

- *(macros)* implement getter for all fields
- *(macros)* apply cargo clippy recommendations
- *(macros)* propagate #[serde(rename)] in #[derive(New)]
- *(macros)* skip unset fields when serializing
- *(macros)* use immutable builder style
- *(macros)* cargo clippy feedback
- ability to require fields in #[derive(New)]
- *(macros)* setter functions for all fields
- *(macros)* create default new()
