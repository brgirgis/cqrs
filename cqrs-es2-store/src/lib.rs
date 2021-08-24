#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic,
    //missing_debug_implementations
)]

//! Sync implementation of the cqrs-es2 store.
//!
//! Provides sync interfaces to different database implementations for
//! the CQRS system store.
//!
//! # Design
//!
//! The main components of this library are:
//!   - `IEventDispatcher` - an interface for sync events listeners
//!   - `IEventStore` - an interface for sync event stores
//!   - `IQueryStore` - an interface for sync query stores
//!
//! # Features
//!
//! - `with-postgres` - sync Postgres store
//! - `with-all-sync` - all sync drivers (default)
//!
//! # Usage
//!
//! To use this library in a sync application, add the following to
//! your dependency section in the project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # logging
//! log = { version = "^0.4", features = [
//!   "max_level_debug",
//!   "release_max_level_warn",
//! ] }
//! fern = "^0.5"
//!
//! # serialization
//! serde = { version = "^1.0.127", features = ["derive"] }
//! serde_json = "^1.0.66"
//!
//! # CQRS framework
//! cqrs-es2 = { version = "*"}
//!
//! # Sync postgres store implementation
//! cqrs-es2-store = { version = "*", default-features = false, features = [
//!   "with-postgres",
//! ] }
//!
//! # postgres driver
//! postgres = { version = "^0.19.1", features = ["with-serde_json-1"] }
//! ```
//!
//! # Example
//!
//! A full sync store example application is available [here](https://github.com/brgirgis/cqrs-es2/tree/master/examples/restful).

pub use repository::*;

#[cfg(feature = "with-postgres")]
pub use sql::*;

/// Sync memory store
pub mod memory_store;

/// Sync repository interface
mod repository;

#[cfg(feature = "with-postgres")]
mod sql;
