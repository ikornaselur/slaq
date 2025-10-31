use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Sends a message to a channel.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.postMessage",
    response=PostMessageResponse,
)]
pub struct PostMessage {
    /// An encoded ID or channel name that represents a channel, private group, or IM channel to send the message to. See below[0] for more details.
    ///
    /// [0] <https://docs.slack.dev/reference/methods/chat.postmessage#channels>
    pub channel: String,
    /// (Legacy) Pass true to post the message as the authed user instead of as a bot. Defaults to false. Can only be used by classic apps. See legacy `as_user` parameter below[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postmessage#legacy_as_user>
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
    /// This field represents the timestamp of the draft's last update at the time this API is called. If the current message is a draft, this field can be provided to ensure synchronization with the server.
    pub current_draft_last_updated_ts: Option<String>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message.
    pub icon_url: Option<String>,
    /// Find and link user groups. No longer supports linking individual users; use syntax shown in Mentioning Users[0] instead.
    ///
    /// [0]: <https://docs.slack.dev/messaging/formatting-message-text/#mentioning-users>
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    pub markdown_text: Option<String>,
    /*
    /// JSON object with event_type and event_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    ///
    /// Note: Should be provided as Metadata
    // pub metadata: Option<Metadata>
     */
    /// Disable Slack markup parsing by setting to `false`. Enabled by default.
    pub mrkdwn: Option<bool>,
    /// Change how messages are treated. See below[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postmessage#formatting>
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. See below[0] for more detail.
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postmessage#text_usage>
    pub text: Option<String>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name.
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.postMessage`.
pub struct PostMessageResponse {}
