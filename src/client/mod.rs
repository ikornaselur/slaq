pub mod method;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "async")]
pub mod r#async;
