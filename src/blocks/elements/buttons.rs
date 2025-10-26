use serde::Serialize;

use crate::blocks::text::PlainText;

use super::common::{BlockElement, ButtonStyle, ConfirmationDialog};

/// Button element definition.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ButtonElement {
    #[serde(rename = "type")]
    kind: ButtonKind,
    pub text: PlainText,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl ButtonElement {
    #[must_use]
    pub fn new(text: PlainText, action_id: impl Into<String>) -> Self {
        Self {
            kind: ButtonKind::Button,
            text,
            action_id: action_id.into(),
            url: None,
            value: None,
            style: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ButtonKind {
    Button,
}

impl From<ButtonElement> for BlockElement {
    fn from(value: ButtonElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

