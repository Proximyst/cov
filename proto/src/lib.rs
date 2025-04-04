//! The protocol for cov.
//!
//! This is a RESTful API. The types are defined such that they can be presented as an OpenAPI3 schema.

pub mod auth;
mod either;
pub mod error;
pub mod health;
pub mod ping;
pub mod version;

pub use either::AxumEither;
