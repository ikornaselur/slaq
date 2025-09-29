use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updates a message.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.update",
    chat_method=update,
    response=UpdateResponse,
    call_alias="UpdateCall"
)]
pub struct Update {
    /// Channel containing the message to be updated. For direct messages, ensure that this value is a DM ID (starts with `D`) instead of a User ID (starts with either `U` or `W`).
    pub channel: String,
    /// Timestamp of the message to be updated.
    pub ts: String,
    /// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
    /*
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    ///
    /// Note: Should be provided as a Vec<Attachment>
    // pub attachments: Option<Vec<Attachment>>,
     */
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    ///
    /// Note: Uses a simplified internal `Block` representation.
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    pub markdown_text: Option<String>,
    /*
    /// JSON object with event_type and event_payload fields, presented as a URL-encoded string. If you don't include this field, the message's previous `metadata` will be retained. To remove previous `metadata`, include an empty object for this field. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    ///
    /// Note: Should be provided as Metadata
    // pub metadata: Option<Metadata>,
     */
    /// Find and link channel names and usernames. Defaults to `none`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `none`.
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `client`.
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. See below[0] for more detail.
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.update#text_usage>
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.update`.
pub struct UpdateResponse {
    /// Channel ID where the message was updated.
    pub channel: String,
    /// Timestamp of the updated message.
    pub ts: String,
}
