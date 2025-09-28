//! Minimal BlockKit-like builders for composing message blocks.
//!
//! This module currently supports two simple block variants:
//! - Divider: `{ "type": "divider", "block_id": "..."? }`
//! - Markdown: `{ "type": "markdown", "text": "...", "block_id": "..."? }`
//!
//! These are intentionally simplified and not a full Block Kit implementation.

use serde::Serialize;
use serde_json as json;

/// A minimal set of supported blocks.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Block(pub json::Value);

#[slaq_macros::block(kind = "divider")]
#[derive(Debug, Clone)]
/// Visually separates pieces of info inside of a message.
pub struct Divider {
    /// A unique identifier for a block. If not specified, one will be generated.
    /// Maximum length for this field is 255 characters. `block_id`` should be unique
    /// for each message and each iteration of a message. If a message is updated,
    /// use a new `block_id`.
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "markdown")]
#[derive(Debug, Clone)]
/// Displays formatted markdown.
pub struct Markdown {
    /// The standard markdown-formatted text. Limit 12,000 characters max.
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_serializes_minimal() {
        let block = Divider::new().build();
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"divider"}"#);
    }

    #[test]
    fn markdown_serializes_with_text_and_block_id() {
        let block = Markdown::new("hello").build();
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"markdown\""));
        assert!(json.contains("\"text\":\"hello\""));
    }
}
