use serde::Serialize;

use crate::blocks::text::PlainText;

use super::common::{BlockElement, ConfirmationDialog, DispatchActionConfig};
use super::selects::SelectMenuKind;

/// Plain text input element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PlainTextInputElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dispatch_action_config: Option<DispatchActionConfig>,
}

impl PlainTextInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::PlainTextInput,
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }

    #[must_use]
    pub fn multiline(mut self, yes: bool) -> Self {
        self.multiline = Some(yes);
        self
    }

    #[must_use]
    pub fn min_length(mut self, value: u32) -> Self {
        self.min_length = Some(value);
        self
    }

    #[must_use]
    pub fn max_length(mut self, value: u32) -> Self {
        self.max_length = Some(value);
        self
    }

    #[must_use]
    pub fn dispatch_action_config(mut self, cfg: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(cfg);
        self
    }
}

/// Date picker element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DatePickerElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl DatePickerElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::DatePicker,
            action_id: action_id.into(),
            placeholder: None,
            initial_date: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_date(mut self, date: impl Into<String>) -> Self {
        self.initial_date = Some(date.into());
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

/// Time picker element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TimePickerElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl TimePickerElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::TimePicker,
            action_id: action_id.into(),
            placeholder: None,
            initial_time: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_time(mut self, time: impl Into<String>) -> Self {
        self.initial_time = Some(time.into());
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

impl From<PlainTextInputElement> for BlockElement {
    fn from(value: PlainTextInputElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<DatePickerElement> for BlockElement {
    fn from(value: DatePickerElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<TimePickerElement> for BlockElement {
    fn from(value: TimePickerElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

