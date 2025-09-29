use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Schedules a message to be sent to a channel.
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.scheduleMessage",
    chat_method=schedule_message,
    response=ScheduleMessageResponse,
    call_alias="ScheduleMessageCall"
)]
pub struct ScheduleMessage {
    /// Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See below[0] for more details.
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.schedulemessage#channels>
    pub channel: String,
    /// Unix timestamp representing the future time the message should post to Slack.
    pub post_at: i64,
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
    /// Find and link user groups. No longer supports linking individual users; use syntax shown in Mentioning Users[0] instead.
    ///
    /// [0]: <https://docs.slack.dev/messaging/formatting-message-text#mentioning-users>
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    pub markdown_text: Option<String>,
    /*
    /// JSON object with event_type and event_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    ///
    /// Note: Should be provided as Metadata
    // pub metadata: Option<Metadata>,
     */
    /// Set to `true` to post the message as the authed user, instead of as a bot. Defaults to false. Cannot be used by new Slack apps. See chat.postMessage[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postmessage#authorship>
    pub as_user: Option<bool>,
    /// Change how messages are treated. See chat.postMessage[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postmessage#formatting>
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. See below[0] for more detail.
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.schedulemessage#text_usage>
    pub text: Option<String>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.scheduleMessage`.
pub struct ScheduleMessageResponse {
    /// Channel ID where the message is scheduled.
    pub channel: String,
    /// Identifier for the scheduled message.
    pub scheduled_message_id: String,
    /// When the message is scheduled to post.
    pub post_at: String,
}
