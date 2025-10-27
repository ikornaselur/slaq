use serde::Serialize;

/// Plain text object used across many Block Kit structures.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlainText {
    #[serde(rename = "type")]
    kind: &'static str,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub emoji: Option<bool>,
}

impl PlainText {
    const KIND: &'static str = "plain_text";

    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            kind: Self::KIND,
            text: text.into(),
            emoji: None,
        }
    }

    #[must_use]
    pub fn emoji(mut self, enable: bool) -> Self {
        self.emoji = Some(enable);
        self
    }
}

impl<T: Into<String>> From<T> for PlainText {
    fn from(value: T) -> Self {
        PlainText::new(value)
    }
}

/// Markdown text object (mrkdwn) for Slack blocks.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MrkdwnText {
    #[serde(rename = "type")]
    kind: &'static str,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verbatim: Option<bool>,
}

impl MrkdwnText {
    const KIND: &'static str = "mrkdwn";

    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            kind: Self::KIND,
            text: text.into(),
            verbatim: None,
        }
    }

    #[must_use]
    pub fn verbatim(mut self, enable: bool) -> Self {
        self.verbatim = Some(enable);
        self
    }
}

impl<T: Into<String>> From<T> for MrkdwnText {
    fn from(value: T) -> Self {
        MrkdwnText::new(value)
    }
}

/// Wrapper enum for text objects that can be either plain text or mrkdwn.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TextObject {
    Plain(PlainText),
    Mrkdwn(MrkdwnText),
}

impl From<PlainText> for TextObject {
    fn from(value: PlainText) -> Self {
        TextObject::Plain(value)
    }
}

impl From<MrkdwnText> for TextObject {
    fn from(value: MrkdwnText) -> Self {
        TextObject::Mrkdwn(value)
    }
}

impl From<&str> for TextObject {
    fn from(value: &str) -> Self {
        TextObject::Mrkdwn(MrkdwnText::new(value))
    }
}

impl From<String> for TextObject {
    fn from(value: String) -> Self {
        TextObject::Mrkdwn(MrkdwnText::new(value))
    }
}

// Namespaced text macros under `slaq::blocks::text::*` for clarity.
#[doc(hidden)]
pub use crate::{fields, mrkdwn, plain};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_serializes() {
        let text = PlainText::new("hello").emoji(true);
        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("\"type\":\"plain_text\""));
        assert!(json.contains("\"text\":\"hello\""));
        assert!(json.contains("\"emoji\":true"));
    }

    #[test]
    fn mrkdwn_serializes() {
        let text = MrkdwnText::new("*hi*");
        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("\"type\":\"mrkdwn\""));
        assert!(json.contains("\"text\":\"*hi*\""));
    }

    #[test]
    fn text_object_from_plain() {
        let text: TextObject = PlainText::new("hello").into();
        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("\"type\":\"plain_text\""));
    }
}
