use serde::Serialize;

use crate::blocks::text::{MrkdwnText, PlainText};

/// Element types that can appear inside a context block.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ContextElement {
    PlainText(PlainText),
    Mrkdwn(MrkdwnText),
    Image(ContextImage),
}

impl ContextElement {
    #[must_use]
    pub fn plain_text(text: impl Into<String>) -> Self {
        ContextElement::PlainText(PlainText::new(text))
    }

    #[must_use]
    pub fn mrkdwn(text: impl Into<String>) -> Self {
        ContextElement::Mrkdwn(MrkdwnText::new(text))
    }

    #[must_use]
    pub fn image(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        ContextElement::Image(ContextImage::new(image_url, alt_text))
    }
}

/// Image element usable within context blocks and accessories.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ContextImage {
    #[serde(rename = "type")]
    kind: ContextImageKind,
    pub image_url: String,
    pub alt_text: String,
}

impl ContextImage {
    #[must_use]
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        Self {
            kind: ContextImageKind::Image,
            image_url: image_url.into(),
            alt_text: alt_text.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ContextImageKind {
    Image,
}

/// Alias for the generic image element usable as a section accessory or elsewhere.
pub type ImageElement = ContextImage;
