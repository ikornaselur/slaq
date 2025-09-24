use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.postMessage",
    chat_method=post_message,
    response=PostMessageResponse,
    call_alias="PostMessageCall"
)]
pub struct PostMessage {
    pub channel: String,
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
pub struct PostMessageResponse {}
