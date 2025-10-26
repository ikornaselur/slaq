use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Returns a list of scheduled messages.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.scheduledMessages.list",
    response=ScheduledMessagesListResponse,
)]
pub struct ScheduledMessagesList {
    /// The channel of the scheduled messages
    pub channel: Option<String>,
    /// For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
    pub cursor: Option<String>,
    /// A Unix timestamp of the latest value in the time range
    pub latest: Option<String>,
    /// Maximum number of original entries to return.
    pub limit: Option<u32>,
    /// A Unix timestamp of the oldest value in the time range
    pub oldest: Option<String>,
    /// encoded team id to list channels in, required if org token is used
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.scheduledMessages.list`.
pub struct ScheduledMessagesListResponse {
    /// List of scheduled messages.
    pub scheduled_messages: Vec<ScheduledMessage>,
    /// Optional response metadata including pagination cursors.
    pub response_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledMessage {
    pub id: i64,
    pub channel_id: String,
    pub post_at: i64,
    pub date_created: i64,
    pub text: String,
}
