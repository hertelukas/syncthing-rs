# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.3](https://github.com/hertelukas/syncthing-rs/compare/syncthing-rs-v0.1.0-alpha.2...syncthing-rs-v0.1.0-alpha.3) - 2025-05-18

### Added

- *(config)* delete folder/device endpoints
- *(db)* get completion endpoint
- *(system)* get connections endpoint
- parsing all events
- #[derive(ParitalEq)] for all config types
- *(config)* everything is clone
- *(events)* create NewDeviceConfiguration from event

### Fixed

- *(events)* fixed typo
- *(events)* change type of folder scan progress
- *(events)* change some types
- *(client)* use & to indicate specific device's folder

### Other

- *(db)* completion fields pub
- *(cluster)* made fields public
- cargo clippy
- use new API
- *(client)* rename delete to dismiss

## [0.1.0-alpha.2](https://github.com/hertelukas/syncthing-rs/compare/syncthing-rs-v0.1.0-alpha.1...syncthing-rs-v0.1.0-alpha.2) - 2025-05-12

### Added

- *(types)* derive New for devices and folders

### Other

- *(client)* avoid flaky test on get_events
- *(client)* clippy
- *(client)* test pending and defaults
- *(client)* test add and post endpoints
- *(client)* setup testcontainer to test agains syncthing
- *(client)* accept for new everything thats into new
