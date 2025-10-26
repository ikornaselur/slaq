use serde::Serialize;
use super::common::{BlockElement, ConfirmationDialog, SelectOption};

/// Checkboxes element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CheckboxesElement {
    #[serde(rename = "type")]
    kind: InputLikeKind,
    pub action_id: String,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl CheckboxesElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>, options: impl Into<Vec<SelectOption>>) -> Self {
        Self {
            kind: InputLikeKind::Checkboxes,
            action_id: action_id.into(),
            options: options.into(),
            initial_options: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn initial_options(mut self, options: impl Into<Vec<SelectOption>>) -> Self {
        self.initial_options = Some(options.into());
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

/// Radio buttons element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RadioButtonsElement {
    #[serde(rename = "type")]
    kind: InputLikeKind,
    pub action_id: String,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_option: Option<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl RadioButtonsElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>, options: impl Into<Vec<SelectOption>>) -> Self {
        Self {
            kind: InputLikeKind::RadioButtons,
            action_id: action_id.into(),
            options: options.into(),
            initial_option: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn initial_option(mut self, option: SelectOption) -> Self {
        self.initial_option = Some(option);
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

/// Overflow menu element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OverflowElement {
    #[serde(rename = "type")]
    kind: OverflowKind,
    pub action_id: String,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl OverflowElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>, options: impl Into<Vec<SelectOption>>) -> Self {
        Self {
            kind: OverflowKind::Overflow,
            action_id: action_id.into(),
            options: options.into(),
            confirm: None,
        }
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum OverflowKind {
    Overflow,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum InputLikeKind {
    Checkboxes,
    RadioButtons,
}

impl From<CheckboxesElement> for BlockElement {
    fn from(value: CheckboxesElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<RadioButtonsElement> for BlockElement {
    fn from(value: RadioButtonsElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<OverflowElement> for BlockElement {
    fn from(value: OverflowElement) -> Self {
        BlockElement::from_struct(&value)
    }
}
