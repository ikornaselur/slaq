use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.postEphemeral",
    chat_method=post_ephemeral,
    response=PostEphemeralResponse,
    call_alias="PostEphemeralCall"
)]
/// Sends an ephemeral message to a user in a channel.
pub struct PostEphemeral {
    pub channel: String,
    pub user: String,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub markdown_text: Option<String>,
    pub mrkdwn: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub text: Option<String>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.postEphemeral`.
pub struct PostEphemeralResponse {
    pub message_ts: String,
}
