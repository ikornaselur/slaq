use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Deletes a message.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.delete",
    chat_method=delete,
    response=DeleteResponse,
    call_alias="DeleteCall"
)]
pub struct Delete {
    /// Channel containing the message to be deleted.
    pub channel: String,
    /// Timestamp of the message to be deleted.
    pub ts: String,
    /// (Legacy) Pass true to delete the message as the authed user with `chat:write:user` scope. Bot users in this context are considered authed users. See legacy `as_user` parameter below[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.delete#legacy_as_user>
    pub as_user: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.delete`.
pub struct DeleteResponse {
    /// Channel ID where the message was deleted.
    pub channel: String,
    /// Timestamp of the deleted message.
    pub ts: String,
}
