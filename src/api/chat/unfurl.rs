use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Provide custom unfurl behavior for user-posted URLs
///
/// Bot token: chat:write
/// User token: chat:write
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[slaq_macros::slack_api(
    path="/chat.unfurl",
    chat_method=unfurl,
    response=UnfurlResponse,
    call_alias="UnfurlCall"
)]
pub struct Unfurl {
    /// Channel ID of the message. Both `channel` and `ts` must be provided together, or `unfurl_id` and `source` must be provided together.
    pub channel: String,
    /// Timestamp of the message to add unfurl behavior to.
    pub ts: String,
    /// URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments.
    pub unfurls: String,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior. Provides two buttons, `Not now` or `Never ask me again`.
    pub user_auth_message: Option<String>,
    /// Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    pub user_auth_url: Option<String>,
    /// Provide a JSON based array of structured blocks presented as URL-encoded string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    ///
    /// Note: Uses a simplified internal `Block` representation.
    pub user_auth_blocks: Option<Vec<crate::blocks::Block>>,
}

#[derive(Debug, Clone, Deserialize)]
/// Response for `chat.unfurl`.
pub struct UnfurlResponse {}
