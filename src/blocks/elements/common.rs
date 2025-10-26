use serde::Serialize;

use crate::blocks::text::{PlainText, TextObject};

/// Generic container for Block Kit interactive elements used across actions, input, and section blocks.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct BlockElement(pub serde_json::Value);

impl BlockElement {
    /// Converts a serializable element struct into a `BlockElement` wrapper.
    ///
    /// # Panics
    /// If serialization of `value` fails (unexpected for well-formed types).
    #[must_use]
    pub fn from_struct<T: Serialize>(value: &T) -> Self {
        let json = serde_json::to_value(value).expect("serialize block element");
        BlockElement(json)
    }
}

/// Available button styles.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Danger,
}

/// Confirmation dialog used by interactive elements.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ConfirmationDialog {
    pub title: PlainText,
    pub text: TextObject,
    pub confirm: PlainText,
    pub deny: PlainText,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub style: Option<ButtonStyle>,
}

impl ConfirmationDialog {
    #[must_use]
    pub fn new(
        title: PlainText,
        text: impl Into<TextObject>,
        confirm: PlainText,
        deny: PlainText,
    ) -> Self {
        Self {
            title,
            text: text.into(),
            confirm,
            deny,
            style: None,
        }
    }

    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// Option entry for select menus.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SelectOption {
    pub text: PlainText,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<PlainText>,
}

impl SelectOption {
    #[must_use]
    pub fn new(text: PlainText, value: impl Into<String>) -> Self {
        Self {
            text,
            value: value.into(),
            description: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: PlainText) -> Self {
        self.description = Some(description);
        self
    }
}

/// Group of options for select menus.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OptionGroup {
    pub label: PlainText,
    pub options: Vec<SelectOption>,
}

impl OptionGroup {
    #[must_use]
    pub fn new(label: PlainText, options: impl Into<Vec<SelectOption>>) -> Self {
        Self {
            label,
            options: options.into(),
        }
    }
}

/// Dispatch action configuration for input elements.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DispatchActionConfig {
    pub trigger_actions_on: Vec<DispatchActionTrigger>,
}

impl DispatchActionConfig {
    #[must_use]
    pub fn new(triggers: impl Into<Vec<DispatchActionTrigger>>) -> Self {
        Self {
            trigger_actions_on: triggers.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DispatchActionTrigger {
    OnEnterPressed,
    OnCharacterEntered,
}
