#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic,
    //missing_debug_implementations
)]

//! # cqrs-es2
//!
//! A Rust library providing lightweight CQRS and event sourcing
//! framework.
//!
//! [![Publish](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml)
//! [![Test](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml)
//! [![Latest version](https://img.shields.io/crates/v/cqrs-es2)](https://crates.io/crates/cqrs-es2)
//! [![docs](https://img.shields.io/badge/API-docs-blue.svg)](https://docs.rs/cqrs-es2)
//! ![License](https://img.shields.io/crates/l/cqrs-es2.svg)
//!
//! ---
//!
//! Provides all basic interfaces to for the CQRS system.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! # serialization
//! serde = { version = "^1.0.127", features = ["derive"] }
//! serde_json = "^1.0.66"
//!
//! # CQRS framework
//! cqrs-es2 = { version = "*"}
//! ```
//!
//! ## Usage
//!
//! Full fledged demo applications:
//!
//! - Sync [RESTful](https://github.com/brgirgis/cqrs-es2-store/tree/master/examples/restful)
//! - Async [gRPC](https://github.com/brgirgis/tokio-cqrs-es2-store/tree/master/examples/grpc)

pub use crate::{
    aggregates::*,
    commands::*,
    errors::*,
    events::*,
    queries::*,
    test_framework::*,
};

/// Errors module holds the library error types.
mod errors;

/// Aggregates module holds the central traits that define the
/// fundamental component of CQRS.
mod aggregates;

/// Commands module provides the abstract domain commands.
mod commands;

/// Events module provides the abstract domain events and associated
/// wrapper.
mod events;

/// Queries module provides the basic downstream query objects needed
/// to render queries (or "views") that describe the state of the
/// system.
mod queries;

/// Test provides a test framework for building a resilient test base
/// around aggregates. A `TestFramework` should be used to build a
/// comprehensive set of aggregate tests to verify your application
/// logic (aka business rules).
mod test_framework;

#[doc(hidden)]
pub mod example_impl;
