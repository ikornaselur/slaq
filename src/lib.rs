pub mod api;
mod blocks;

pub const DEFAULT_BASE_URL: &str = "https://slack.com/api";
pub mod client;
pub use client::Client;
pub mod prelude;
