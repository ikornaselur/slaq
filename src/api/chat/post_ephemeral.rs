use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Chat;
use crate::client::{Execute, SlackMethod};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
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

impl PostEphemeral {
    #[must_use]
    pub fn new(channel: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            user: user.into(),
            ..Default::default()
        }
    }
    #[must_use]
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    #[must_use]
    pub fn icon_emoji(mut self, v: impl Into<String>) -> Self {
        self.icon_emoji = Some(v.into());
        self
    }
    #[must_use]
    pub fn icon_url(mut self, v: impl Into<String>) -> Self {
        self.icon_url = Some(v.into());
        self
    }
    #[must_use]
    pub fn link_names(mut self, v: bool) -> Self {
        self.link_names = Some(v);
        self
    }
    #[must_use]
    pub fn markdown_text(mut self, v: impl Into<String>) -> Self {
        self.markdown_text = Some(v.into());
        self
    }
    #[must_use]
    pub fn mrkdwn(mut self, v: bool) -> Self {
        self.mrkdwn = Some(v);
        self
    }
    #[must_use]
    pub fn parse(mut self, v: impl Into<String>) -> Self {
        self.parse = Some(v.into());
        self
    }
    #[must_use]
    pub fn reply_broadcast(mut self, v: bool) -> Self {
        self.reply_broadcast = Some(v);
        self
    }
    #[must_use]
    pub fn thread_ts(mut self, v: impl Into<String>) -> Self {
        self.thread_ts = Some(v.into());
        self
    }
    #[must_use]
    pub fn unfurl_links(mut self, v: bool) -> Self {
        self.unfurl_links = Some(v);
        self
    }
    #[must_use]
    pub fn unfurl_media(mut self, v: bool) -> Self {
        self.unfurl_media = Some(v);
        self
    }
    #[must_use]
    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.username = Some(v.into());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostEphemeralResponse {
    pub message_ts: String,
}

impl SlackMethod for PostEphemeral {
    const PATH: &'static str = "/chat.postEphemeral";
    type Body = Self;
    type Response = PostEphemeralResponse;
    fn into_body(self) -> Self::Body {
        self
    }
}

pub struct PostEphemeralCall<'a, C: Execute> {
    pub(crate) client: &'a C,
    pub(crate) inner: PostEphemeral,
}

impl<C: Execute> PostEphemeralCall<'_, C> {
    #[must_use]
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.text(v);
        self
    }
    #[must_use]
    pub fn icon_emoji(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.icon_emoji(v);
        self
    }
    #[must_use]
    pub fn icon_url(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.icon_url(v);
        self
    }
    #[must_use]
    pub fn link_names(mut self, v: bool) -> Self {
        self.inner = self.inner.link_names(v);
        self
    }
    #[must_use]
    pub fn markdown_text(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.markdown_text(v);
        self
    }
    #[must_use]
    pub fn mrkdwn(mut self, v: bool) -> Self {
        self.inner = self.inner.mrkdwn(v);
        self
    }
    #[must_use]
    pub fn parse(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.parse(v);
        self
    }
    #[must_use]
    pub fn reply_broadcast(mut self, v: bool) -> Self {
        self.inner = self.inner.reply_broadcast(v);
        self
    }
    #[must_use]
    pub fn thread_ts(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.thread_ts(v);
        self
    }
    #[must_use]
    pub fn unfurl_links(mut self, v: bool) -> Self {
        self.inner = self.inner.unfurl_links(v);
        self
    }
    #[must_use]
    pub fn unfurl_media(mut self, v: bool) -> Self {
        self.inner = self.inner.unfurl_media(v);
        self
    }
    #[must_use]
    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.inner = self.inner.username(v);
        self
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn send(self) -> Result<PostEphemeralResponse, C::Error> {
        self.client.execute(self.inner)
    }
}

impl<'a, C: Execute> Chat<'a, C> {
    pub fn post_ephemeral(
        &'a self,
        channel: impl Into<String>,
        user: impl Into<String>,
    ) -> PostEphemeralCall<'a, C> {
        PostEphemeralCall {
            client: self.client,
            inner: PostEphemeral::new(channel, user),
        }
    }
}
