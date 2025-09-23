pub mod api;
mod blocks;

pub const DEFAULT_BASE_URL: &str = "https://slack.com/api";
pub mod client;

#[cfg(feature = "blocking")]
pub use client::blocking::Client as BlockingClient;

#[cfg(feature = "async")]
pub use client::r#async::Client as AsyncClient;
