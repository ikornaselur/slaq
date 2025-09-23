use serde::Serialize;
use serde_json::{self, Value};
use serde_with::skip_serializing_none;

pub const PATH: &str = "/chat.postMessage";

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize)]
pub struct PostMessagePayload {
    channel: String,
    // attachments: Option<Vec<Attachment>>,
    // blocks: Option<Vec<Block>>,
    icon_emoji: Option<String>,
    icon_url: Option<String>,
    link_names: Option<bool>,
    markdown_text: Option<String>,
    // metadata: Option<Metadata>,
    mrkdwn: Option<bool>,
    parse: Option<String>,
    reply_broadcast: Option<bool>,
    text: Option<String>,
    thread_ts: Option<String>,
    unfurl_links: Option<bool>,
    unfurl_media: Option<bool>,
    username: Option<String>,
}

impl PostMessagePayload {
    pub fn to_json_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn into_value(self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone)]
pub struct PostMessagePayloadBuilder {
    inner: PostMessagePayload,
    path: String,
}

impl PostMessagePayloadBuilder {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            inner: PostMessagePayload {
                channel: channel.into(),
                ..Default::default()
            },
        }
    }

    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.inner.text = Some(v.into());
        self
    }
    pub fn icon_emoji(mut self, v: impl Into<String>) -> Self {
        self.inner.icon_emoji = Some(v.into());
        self
    }
    pub fn icon_url(mut self, v: impl Into<String>) -> Self {
        self.inner.icon_url = Some(v.into());
        self
    }
    pub fn link_names(mut self, v: bool) -> Self {
        self.inner.link_names = Some(v);
        self
    }
    pub fn markdown_text(mut self, v: impl Into<String>) -> Self {
        self.inner.markdown_text = Some(v.into());
        self
    }
    pub fn mrkdwn(mut self, v: bool) -> Self {
        self.inner.mrkdwn = Some(v);
        self
    }
    pub fn parse(mut self, v: impl Into<String>) -> Self {
        self.inner.parse = Some(v.into());
        self
    }
    pub fn reply_broadcast(mut self, v: bool) -> Self {
        self.inner.reply_broadcast = Some(v);
        self
    }
    pub fn thread_ts(mut self, v: impl Into<String>) -> Self {
        self.inner.thread_ts = Some(v.into());
        self
    }
    pub fn unfurl_links(mut self, v: bool) -> Self {
        self.inner.unfurl_links = Some(v);
        self
    }
    pub fn unfurl_media(mut self, v: bool) -> Self {
        self.inner.unfurl_media = Some(v);
        self
    }
    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.inner.username = Some(v.into());
        self
    }

    pub fn build(self) -> PostMessagePayload {
        self.inner
    }

    pub fn build_json(self) -> serde_json::Result<String> {
        self.build().to_json_string()
    }

    pub fn build_value(self) -> serde_json::Result<Value> {
        self.build().into_value()
    }
}
