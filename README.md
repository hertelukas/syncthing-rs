# syncthing-rs

[![Crates.io Version](https://img.shields.io/crates/v/syncthing-rs)](https://crates.io/crates/syncthing-rs)
[![docs.rs](https://img.shields.io/docsrs/syncthing-rs)](https://docs.rs/syncthing-rs/)
[![Crates.io License](https://img.shields.io/crates/l/syncthing-rs)](./LICENSE-APACHE)
[![CI](https://github.com/hertelukas/syncthing-rs/workflows/CI/badge.svg)](https://github.com/hertelukas/syncthing-rs/actions?query=workflow%3ACI)

> [!WARNING]
> This is an incomplete wrapper for the [Syncthing REST API](https://docs.syncthing.net/dev/rest.html), under active development.  
> I'm not aiming to support the full API — just the parts I need for my own projects.

> [!NOTE]
> - Not affiliated with the Syncthing Foundation.
> - Contributions are welcome!

A wrapper around the [Syncthing REST API](https://docs.syncthing.net/dev/rest.html), with extra sauce.  
Targeting the latest **stable** version of [Syncthing](https://syncthing.net).

## Roadmap
- [X] Core configuration endpoint 
- [X] Configuration data types
- [ ] Partial updates to configuration
- [X] Event subscription
- [ ] Full event parsing
- [X] Cluster endpoints and data types
- [ ] Support for multiple API versions
