use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Sends an ephemeral message to a user in a channel.
///
/// Bot token: `chat:write`
/// User token: `chat:write`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.postEphemeral",
    response=PostEphemeralResponse,
)]
pub struct PostEphemeral {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument.
    pub user: String,
    /// (Legacy) Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false.
    pub as_user: Option<bool>,
    /*
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    ///
    /// Note: Should be provided as a Vec<Attachment>
    // pub attachments: Option<Vec<Attachment>>,
     */
    /// A JSON-based array of structured blocks.
    ///
    /// Note: Uses a simplified internal `Block` representation.
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message.
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    pub markdown_text: Option<String>,
    /// Change how messages are treated. Defaults to `none`. See below[0].
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postephemeral#formatting>
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. See below[0] for more detail.
    ///
    /// [0]: <https://docs.slack.dev/reference/methods/chat.postephemeral#text_usage>
    pub text: Option<String>,
    /// Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread.
    pub thread_ts: Option<String>,
    /// Set your bot's user name.
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.postEphemeral`.
pub struct PostEphemeralResponse {
    /// Timestamp of the ephemeral message that was posted.
    pub message_ts: String,
}
