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

/// Email input element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EmailInputElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_value: Option<String>,
}

impl EmailInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::EmailTextInput,
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
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
}

impl From<EmailInputElement> for BlockElement {
    fn from(value: EmailInputElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

/// URL input element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct UrlInputElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_value: Option<String>,
}

impl UrlInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::UrlTextInput,
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
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
}

impl From<UrlInputElement> for BlockElement {
    fn from(value: UrlInputElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

/// Number input element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NumberInputElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_decimal_allowed: Option<bool>,
}

impl NumberInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::NumberInput,
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
            min_value: None,
            max_value: None,
            is_decimal_allowed: None,
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
    pub fn min_value(mut self, value: i64) -> Self {
        self.min_value = Some(value);
        self
    }

    #[must_use]
    pub fn max_value(mut self, value: i64) -> Self {
        self.max_value = Some(value);
        self
    }

    #[must_use]
    pub fn decimal_allowed(mut self, yes: bool) -> Self {
        self.is_decimal_allowed = Some(yes);
        self
    }
}

impl From<NumberInputElement> for BlockElement {
    fn from(value: NumberInputElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

/// Datetime picker element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DateTimePickerElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_date_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl DateTimePickerElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::DateTimePicker,
            action_id: action_id.into(),
            initial_date_time: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn initial_date_time(mut self, ts: i64) -> Self {
        self.initial_date_time = Some(ts);
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

impl From<DateTimePickerElement> for BlockElement {
    fn from(value: DateTimePickerElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

/// Rich text input element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RichTextInputElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
}

impl RichTextInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self { kind: SelectMenuKind::RichTextInput, action_id: action_id.into() }
    }
}

impl From<RichTextInputElement> for BlockElement {
    fn from(value: RichTextInputElement) -> Self {
        BlockElement::from_struct(&value)
    }
}
