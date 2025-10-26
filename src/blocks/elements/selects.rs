use serde::Serialize;

use crate::blocks::text::PlainText;

use super::common::{BlockElement, ConfirmationDialog, OptionGroup, SelectOption};

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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) enum SelectMenuKind {
    #[serde(rename = "static_select")]
    StaticSelect,
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect,
    #[serde(rename = "external_select")]
    ExternalSelect,
    #[serde(rename = "multi_external_select")]
    MultiExternalSelect,
    #[serde(rename = "users_select")]
    UsersSelect,
    #[serde(rename = "multi_users_select")]
    MultiUsersSelect,
    #[serde(rename = "conversations_select")]
    ConversationsSelect,
    #[serde(rename = "multi_conversations_select")]
    MultiConversationsSelect,
    #[serde(rename = "channels_select")]
    ChannelsSelect,
    #[serde(rename = "multi_channels_select")]
    MultiChannelsSelect,
    #[serde(rename = "plain_text_input")]
    PlainTextInput,
    #[serde(rename = "datepicker")]
    DatePicker,
    #[serde(rename = "timepicker")]
    TimePicker,
    #[serde(rename = "email_text_input")]
    EmailTextInput,
    #[serde(rename = "url_text_input")]
    UrlTextInput,
    #[serde(rename = "number_input")]
    NumberInput,
    #[serde(rename = "datetimepicker")]
    DateTimePicker,
    #[serde(rename = "rich_text_input")]
    RichTextInput,
}

/// External data source select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ExternalSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_query_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_option: Option<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl ExternalSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::ExternalSelect,
            action_id: action_id.into(),
            placeholder: None,
            min_query_length: None,
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
    pub fn min_query_length(mut self, value: u32) -> Self {
        self.min_query_length = Some(value);
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

/// Multi external data source select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MultiExternalSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_query_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl MultiExternalSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::MultiExternalSelect,
            action_id: action_id.into(),
            placeholder: None,
            min_query_length: None,
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
    pub fn min_query_length(mut self, value: u32) -> Self {
        self.min_query_length = Some(value);
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

/// Multi users select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MultiUsersSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl MultiUsersSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::MultiUsersSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_users: None,
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
    pub fn initial_users(mut self, users: impl Into<Vec<String>>) -> Self {
        self.initial_users = Some(users.into());
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

/// Multi conversations select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MultiConversationsSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_conversations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filter: Option<ConversationsFilter>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl MultiConversationsSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::MultiConversationsSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_conversations: None,
            max_selected_items: None,
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
    pub fn initial_conversations(mut self, ids: impl Into<Vec<String>>) -> Self {
        self.initial_conversations = Some(ids.into());
        self
    }

    #[must_use]
    pub fn max_selected_items(mut self, value: u32) -> Self {
        self.max_selected_items = Some(value);
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

/// Multi channels select element.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MultiChannelsSelectElement {
    #[serde(rename = "type")]
    kind: SelectMenuKind,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub placeholder: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initial_channels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub confirm: Option<ConfirmationDialog>,
}

impl MultiChannelsSelectElement {
    #[must_use]
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            kind: SelectMenuKind::MultiChannelsSelect,
            action_id: action_id.into(),
            placeholder: None,
            initial_channels: None,
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
    pub fn initial_channels(mut self, ids: impl Into<Vec<String>>) -> Self {
        self.initial_channels = Some(ids.into());
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

impl Default for ConversationsFilter {
    fn default() -> Self {
        Self::new()
    }
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

impl From<ExternalSelectElement> for BlockElement {
    fn from(value: ExternalSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<MultiExternalSelectElement> for BlockElement {
    fn from(value: MultiExternalSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<MultiUsersSelectElement> for BlockElement {
    fn from(value: MultiUsersSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<MultiConversationsSelectElement> for BlockElement {
    fn from(value: MultiConversationsSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}

impl From<MultiChannelsSelectElement> for BlockElement {
    fn from(value: MultiChannelsSelectElement) -> Self {
        BlockElement::from_struct(&value)
    }
}
