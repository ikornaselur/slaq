//! Slaq: Typed Slack Web API builder with optional transport.
//!
//! By default includes a blocking reqwest-based client.
//! Disable default features for a build-only crate.
pub mod api;
mod blocks;

/// Default Slack Web API base URL.
pub const DEFAULT_BASE_URL: &str = "https://slack.com/api";
pub mod client;
#[cfg(feature = "transport-reqwest")]
pub use client::Client;
pub mod prelude;
