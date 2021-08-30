# cqrs-es2

**A Rust library providing lightweight CQRS and event sourcing framework.**

[![Publish](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml)
[![Test](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml)
[![Latest version](https://img.shields.io/crates/v/cqrs-es2)](https://crates.io/crates/cqrs-es2)
[![docs](https://img.shields.io/badge/API-docs-blue.svg)](https://docs.rs/cqrs-es2)
![License](https://img.shields.io/crates/l/cqrs-es2.svg)

---

Provides all basic interfaces for the CQRS system.

## Installation

```toml
[dependencies]
# serialization
serde = { version = "^1.0.127", features = ["derive"] }
serde_json = "^1.0.66"

# CQRS framework
cqrs-es2 = { version = "*"}
```

## Usage

Full fledged demo applications:

- Sync RESTful [examples](https://github.com/brgirgis/cqrs-es2-store/tree/master/examples)
- Async gRPC [examples](https://github.com/brgirgis/tokio-cqrs-es2-store/tree/master/examples)

## Change Log

A complete history of the change log can be found [here](https://github.com/brgirgis/cqrs-es2/blob/master/ChangeLog.md)

## TODO

An up-to-date list of development aspirations can be found [here](https://github.com/brgirgis/cqrs-es2/blob/master/TODO.md)
