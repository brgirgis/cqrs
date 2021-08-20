#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
// #![warn(clippy::pedantic,missing_debug_implementations)]

//! # cqrs-es2
//!
//! **A Rust library providing lightweight CQRS and event sourcing
//! framework.**
//!
//! [![Publish](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/crates-io.yml)
//! [![Test](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/brgirgis/cqrs-es2/actions/workflows/rust-ci.yml)
//! [![Latest version](https://img.shields.io/crates/v/cqrs-es2)](https://crates.io/crates/cqrs-es2)
//! [![docs](https://img.shields.io/badge/API-docs-blue.svg)](https://docs.rs/cqrs-es2)
//! ![License](https://img.shields.io/crates/l/cqrs-es2.svg)
//!
//! ---
//!
//! ## Usage
//!
//! Full fledged demo applications:
//!
//! - [RESTful](https://github.com/brgirgis/cqrs-restful-demo)
//! - [gRPC](https://github.com/brgirgis/cqrs-grpc-demo)
