//! Minimal BlockKit-like builders for composing message blocks.
//!
//! This module currently supports two simple block variants:
//! - Divider: `{ "type": "divider", "block_id": "..."? }`
//! - Markdown: `{ "type": "markdown", "text": "...", "block_id": "..."? }`
//!
//! These are intentionally simplified and not a full Block Kit implementation.

use serde::Serialize;
use serde_with::skip_serializing_none;

/// A minimal set of supported blocks.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Block {
    Divider(Divider),
    Markdown(Markdown),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
/// A horizontal divider block.
pub struct Divider {
    #[serde(rename = "type")]
    type_: &'static str,
    pub block_id: Option<String>,
}

impl Divider {
    #[must_use]
    pub fn builder() -> DividerBuilder {
        DividerBuilder { block_id: None }
    }
}

/// Builder for `Divider` blocks.
#[derive(Debug, Clone, Default)]
pub struct DividerBuilder {
    block_id: Option<String>,
}

impl DividerBuilder {
    #[must_use]
    pub fn block_id(mut self, v: impl Into<String>) -> Self {
        self.block_id = Some(v.into());
        self
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::Divider(Divider { type_: "divider", block_id: self.block_id })
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
/// A simple markdown block with plain text.
pub struct Markdown {
    #[serde(rename = "type")]
    type_: &'static str,
    pub text: String,
    pub block_id: Option<String>,
}

impl Markdown {
    #[must_use]
    pub fn builder(text: impl Into<String>) -> MarkdownBuilder {
        MarkdownBuilder { text: text.into(), block_id: None }
    }
}

/// Builder for `Markdown` blocks.
#[derive(Debug, Clone)]
pub struct MarkdownBuilder {
    text: String,
    block_id: Option<String>,
}

impl MarkdownBuilder {
    #[must_use]
    pub fn block_id(mut self, v: impl Into<String>) -> Self {
        self.block_id = Some(v.into());
        self
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::Markdown(Markdown { type_: "markdown", text: self.text, block_id: self.block_id })
    }
}

/// Convenience constructor for a divider block builder.
#[must_use]
pub fn divider() -> DividerBuilder {
    Divider::builder()
}

/// Convenience constructor for a markdown block builder.
#[must_use]
pub fn markdown(text: impl Into<String>) -> MarkdownBuilder {
    Markdown::builder(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_serializes_minimal() {
        let block = divider().build();
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"divider"}"#);
    }

    #[test]
    fn markdown_serializes_with_text_and_block_id() {
        let block = markdown("hello").block_id("b1").build();
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"markdown\""));
        assert!(json.contains("\"text\":\"hello\""));
        assert!(json.contains("\"block_id\":\"b1\""));
    }
}
