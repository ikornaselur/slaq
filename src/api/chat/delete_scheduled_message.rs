use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Deletes a pending scheduled message from the queue.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.deleteScheduledMessage",
    chat_method=delete_scheduled_message,
    response=DeleteScheduledMessageResponse,
    call_alias="DeleteScheduledMessageCall"
)]
pub struct DeleteScheduledMessage {
    /// The channel the `scheduled_message` is posting to
    pub channel: String,
    /// `scheduled_message_id` returned from call to chat.scheduleMessage
    pub scheduled_message_id: String,
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. Bot users in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.deleteScheduledMessage`.
pub struct DeleteScheduledMessageResponse {}
