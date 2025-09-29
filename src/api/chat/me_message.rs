use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Share a me message into a channel.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.meMessage",
    chat_method=me_message,
    response=MeMessageResponse,
    call_alias="MeMessageCall"
)]
pub struct MeMessage {
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: String,
    /// Text of the message to send.
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.meMessage`.
pub struct MeMessageResponse {
    /// Channel ID where the message was posted.
    pub channel: String,
    /// Timestamp of the posted message.
    pub ts: String,
}

