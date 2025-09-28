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

#[cfg(test)]
mod macro_tests {
    use crate::client::SlackMethod;

    #[derive(Debug, Clone, serde::Deserialize)]
    struct DummyResp;

    #[slaq_macros::slack_api(
        path="/dummy.method",
        chat_method=dummy_call,
        response=DummyResp,
        call_alias="DummyCall"
    )]
    #[derive(Debug, Clone, serde::Serialize, Default)]
    struct DummyMethod {
        a: String,
        b: Option<u32>,
        c: Option<bool>,
    }

    #[test]
    fn macro_generates_builder_and_build_request() {
        let method = DummyMethod::new("x").b(10u32).c(true);
        assert_eq!(<DummyMethod as SlackMethod>::PATH, "/dummy.method");
        let json = serde_json::to_string(&method).unwrap();
        assert!(json.contains("\"a\":\"x\""));
        assert!(json.contains("\"b\":10"));
        assert!(json.contains("\"c\":true"));
        let built = method.build_request().to_json().unwrap();
        assert!(built.contains("\"a\":\"x\""));
        assert!(built.contains("\"b\":10"));
        assert!(built.contains("\"c\":true"));
    }
}
