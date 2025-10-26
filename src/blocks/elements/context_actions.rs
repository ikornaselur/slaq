use crate::blocks::text::PlainText;
use serde::Serialize;

/// Element types allowed within context actions blocks.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContextActionElement {
    FeedbackButtons(FeedbackButtons),
    IconButton(IconButton),
}

impl ContextActionElement {
    #[must_use]
    pub fn feedback(
        action_id: impl Into<String>,
        positive: FeedbackButton,
        negative: FeedbackButton,
    ) -> Self {
        ContextActionElement::FeedbackButtons(FeedbackButtons::new(action_id, positive, negative))
    }

    #[must_use]
    pub fn icon(icon: impl Into<String>, text: PlainText, action_id: impl Into<String>) -> Self {
        ContextActionElement::IconButton(IconButton::new(icon, text, action_id))
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FeedbackButtons {
    pub action_id: String,
    pub positive_button: FeedbackButton,
    pub negative_button: FeedbackButton,
}

impl FeedbackButtons {
    #[must_use]
    pub fn new(
        action_id: impl Into<String>,
        positive: FeedbackButton,
        negative: FeedbackButton,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            positive_button: positive,
            negative_button: negative,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FeedbackButton {
    pub text: PlainText,
    pub value: String,
}

impl FeedbackButton {
    #[must_use]
    pub fn new(text: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            text: PlainText::new(text),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct IconButton {
    pub icon: String,
    pub text: PlainText,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

impl IconButton {
    #[must_use]
    pub fn new(icon: impl Into<String>, text: PlainText, action_id: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            text,
            action_id: action_id.into(),
            value: None,
        }
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}
