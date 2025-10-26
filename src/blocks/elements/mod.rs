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

/// Alias for the generic image element usable as a section accessory or elsewhere.
pub type ImageElement = ContextImage;

impl From<ContextImage> for BlockElement {
    fn from(value: ContextImage) -> Self {
        BlockElement::from_struct(&value)
    }
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

impl From<MultiStaticSelectElement> for BlockElement {
    fn from(value: MultiStaticSelectElement) -> Self {
        BlockElement::from_struct(&value)
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

impl From<UsersSelectElement> for BlockElement {
    fn from(value: UsersSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<ConversationsSelectElement> for BlockElement {
    fn from(value: ConversationsSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<ChannelsSelectElement> for BlockElement {
    fn from(value: ChannelsSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<FileInputElement> for BlockElement {
    fn from(value: FileInputElement) -> Self {
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
    pub option_groups: Option<Vec<OptionGroup>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_option: Option<SelectOption>,
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
            option_groups: None,
            initial_option: None,
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
    pub fn option_groups(mut self, groups: impl Into<Vec<OptionGroup>>) -> Self {
        self.option_groups = Some(groups.into());
        self
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

#[derive(Debug, Clone, Serialize, PartialEq)]
enum SelectMenuKind {
    #[serde(rename = "static_select")]
    StaticSelect,
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect,
    #[serde(rename = "users_select")]
    UsersSelect,
    #[serde(rename = "conversations_select")]
    ConversationsSelect,
    #[serde(rename = "channels_select")]
    ChannelsSelect,
    #[serde(rename = "plain_text_input")]
    PlainTextInput,
    #[serde(rename = "datepicker")]
    DatePicker,
    #[serde(rename = "timepicker")]
    TimePicker,
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

/// Multi static select menu element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MultiStaticSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub option_groups: Option<Vec<OptionGroup>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl MultiStaticSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::MultiStaticSelect,
            action_id: action_id.into(),
            placeholder: None,
            options: None,
            option_groups: None,
            initial_options: None,
            max_selected_items: None,
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
    pub fn option_groups(mut self, groups: impl Into<Vec<OptionGroup>>) -> Self {
        self.option_groups = Some(groups.into());
        self
    }

    #[must_use]
    pub fn initial_options(mut self, options: impl Into<Vec<SelectOption>>) -> Self {
        self.initial_options = Some(options.into());
        self
    }

    #[must_use]
    pub fn max_selected_items(mut self, value: u32) -> Self {
        self.max_selected_items = Some(value);
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

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

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum InputLikeKind {
    Checkboxes,
    RadioButtons,
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

/// File input element (for Input blocks).
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FileInputElement {
    #[serde(rename = "type")]
    kind: FileInputKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filetypes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_files: Option<u32>,
}

impl FileInputElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: FileInputKind::FileInput,
            action_id: action_id.into(),
            placeholder: None,
            filetypes: None,
            max_files: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn filetypes(mut self, kinds: impl Into<Vec<String>>) -> Self {
        self.filetypes = Some(kinds.into());
        self
    }

    #[must_use]
    pub fn max_files(mut self, value: u32) -> Self {
        self.max_files = Some(value);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum FileInputKind {
    FileInput,
}

/// Users select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct UsersSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl UsersSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::UsersSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_user: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_user(mut self, user: impl Into<String>) -> Self {
        self.initial_user = Some(user.into());
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

/// Filter for conversations select element.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ConversationsFilter {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub include: Option<Vec<ConversationsFilterKind>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exclude_bot_users: Option<bool>,
}

impl ConversationsFilter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            include: None,
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
        }
    }

    #[must_use]
    pub fn include(mut self, kinds: impl Into<Vec<ConversationsFilterKind>>) -> Self {
        self.include = Some(kinds.into());
        self
    }

    #[must_use]
    pub fn exclude_external_shared_channels(mut self, yes: bool) -> Self {
        self.exclude_external_shared_channels = Some(yes);
        self
    }

    #[must_use]
    pub fn exclude_bot_users(mut self, yes: bool) -> Self {
        self.exclude_bot_users = Some(yes);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationsFilterKind {
    Im,
    Mpim,
    Private,
    Public,
}

/// Conversations select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ConversationsSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_to_current_conversation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub response_url_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filter: Option<ConversationsFilter>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl ConversationsSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::ConversationsSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_conversation: None,
            default_to_current_conversation: None,
            response_url_enabled: None,
            filter: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_conversation(mut self, id: impl Into<String>) -> Self {
        self.initial_conversation = Some(id.into());
        self
    }

    #[must_use]
    pub fn default_to_current_conversation(mut self, yes: bool) -> Self {
        self.default_to_current_conversation = Some(yes);
        self
    }

    #[must_use]
    pub fn response_url_enabled(mut self, yes: bool) -> Self {
        self.response_url_enabled = Some(yes);
        self
    }

    #[must_use]
    pub fn filter(mut self, filter: ConversationsFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

/// Channels select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ChannelsSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl ChannelsSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::ChannelsSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_channel: None,
            confirm: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, text: PlainText) -> Self {
        self.placeholder = Some(text);
        self
    }

    #[must_use]
    pub fn initial_channel(mut self, id: impl Into<String>) -> Self {
        self.initial_channel = Some(id.into());
        self
    }

    #[must_use]
    pub fn confirm(mut self, dialog: ConfirmationDialog) -> Self {
        self.confirm = Some(dialog);
        self
    }
}

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
            .options(vec![SelectOption::new(PlainText::new("A"), "a")])
            .initial_option(SelectOption::new(PlainText::new("A"), "a"));
        let json = serde_json::to_string(&select).unwrap();
        assert!(json.contains("\"type\":\"static_select\""));
        assert!(json.contains("Pick one"));
    }

    #[test]
    fn multi_static_select_serializes() {
        let ms = MultiStaticSelectElement::new("pick-many")
            .placeholder(PlainText::new("Pick many"))
            .options(vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ])
            .max_selected_items(2);
        let json = serde_json::to_string(&ms).unwrap();
        assert!(json.contains("\"type\":\"multi_static_select\""));
        assert!(json.contains("Pick many"));
    }

    #[test]
    fn plain_text_input_serializes() {
        let input = PlainTextInputElement::new("comment")
            .placeholder(PlainText::new("Add a comment"))
            .multiline(true)
            .min_length(0)
            .max_length(2000);
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"type\":\"plain_text_input\""));
        assert!(json.contains("Add a comment"));
    }

    #[test]
    fn datepicker_serializes() {
        let d = DatePickerElement::new("date").initial_date("2025-01-01");
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"type\":\"datepicker\""));
        assert!(json.contains("2025-01-01"));
    }

    #[test]
    fn timepicker_serializes() {
        let t = TimePickerElement::new("time").initial_time("13:37");
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("\"type\":\"timepicker\""));
        assert!(json.contains("13:37"));
    }

    #[test]
    fn users_select_serializes() {
        let u = UsersSelectElement::new("user").initial_user("U123");
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"type\":\"users_select\""));
        assert!(json.contains("U123"));
    }

    #[test]
    fn conversations_select_serializes() {
        let c = ConversationsSelectElement::new("conv")
            .placeholder(PlainText::new("Pick"))
            .default_to_current_conversation(true)
            .response_url_enabled(true)
            .filter(
                ConversationsFilter::new()
                    .include(vec![ConversationsFilterKind::Im, ConversationsFilterKind::Private])
                    .exclude_bot_users(true),
            );
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"type\":\"conversations_select\""));
        assert!(json.contains("\"include\""));
    }

    #[test]
    fn channels_select_serializes() {
        let ch = ChannelsSelectElement::new("chan").initial_channel("C123");
        let json = serde_json::to_string(&ch).unwrap();
        assert!(json.contains("\"type\":\"channels_select\""));
        assert!(json.contains("C123"));
    }

    #[test]
    fn file_input_serializes() {
        let f = FileInputElement::new("files").max_files(3);
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("\"type\":\"file_input\""));
        assert!(json.contains("\"max_files\":3"));
    }

    #[test]
    fn checkboxes_serializes() {
        let cb = CheckboxesElement::new(
            "checks",
            vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ],
        )
        .initial_options(vec![SelectOption::new(PlainText::new("B"), "b")]);
        let json = serde_json::to_string(&cb).unwrap();
        assert!(json.contains("\"type\":\"checkboxes\""));
        assert!(json.contains("\"value\":\"a\""));
    }

    #[test]
    fn radio_buttons_serializes() {
        let rb = RadioButtonsElement::new(
            "choice",
            vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ],
        )
        .initial_option(SelectOption::new(PlainText::new("A"), "a"));
        let json = serde_json::to_string(&rb).unwrap();
        assert!(json.contains("\"type\":\"radio_buttons\""));
        assert!(json.contains("\"value\":\"a\""));
    }

    #[test]
    fn overflow_serializes() {
        let ov = OverflowElement::new(
            "more",
            vec![
                SelectOption::new(PlainText::new("Edit"), "edit"),
                SelectOption::new(PlainText::new("Delete"), "delete"),
            ],
        );
        let json = serde_json::to_string(&ov).unwrap();
        assert!(json.contains("\"type\":\"overflow\""));
        assert!(json.contains("\"value\":\"edit\""));
    }
}
