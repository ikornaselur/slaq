use serde::Serialize;

use crate::blocks::text::{MrkdwnText, PlainText, TextObject};

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

/// Generic container for Block Kit interactive elements used across actions, input, and section blocks.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct BlockElement(pub serde_json::Value);

impl BlockElement {
    #[must_use]
    pub fn from_struct<T: Serialize>(value: &T) -> Self {
        let json = serde_json::to_value(value).expect("serialize block element");
        BlockElement(json)
    }
}

impl From<ButtonElement> for BlockElement {
    fn from(value: ButtonElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<StaticSelectElement> for BlockElement {
    fn from(value: StaticSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

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

/// Available button styles.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Danger,
}

/// Static select menu element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct StaticSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl StaticSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::StaticSelect,
            action_id: action_id.into(),
            placeholder: None,
            options: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn options(mut self, options: impl Into<Vec<SelectOption>>) -> Self {
        self.options = Some(options.into());
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
enum SelectMenuKind {
    StaticSelect,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_element_serializes() {
        let button = ButtonElement::new(PlainText::new("Go"), "go").style(ButtonStyle::Primary);
        let json = serde_json::to_string(&button).unwrap();
        assert!(json.contains("\"type\":\"button\""));
        assert!(json.contains("\"style\":\"primary\""));
    }

    #[test]
    fn static_select_serializes() {
        let select = StaticSelectElement::new("pick")
            .placeholder(PlainText::new("Pick one"))
            .options(vec![SelectOption::new(PlainText::new("A"), "a")]);
        let json = serde_json::to_string(&select).unwrap();
        assert!(json.contains("\"type\":\"static_select\""));
        assert!(json.contains("Pick one"));
    }
}
