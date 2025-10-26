use serde::Serialize;

use crate::blocks::text::PlainText;
use crate::blocks::BuildError;

fn is_https(url: &str) -> bool { url.starts_with("https://") }

// https://docs.slack.dev/reference/block-kit/blocks/video-block
#[slaq_macros::block(kind = "video", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Video {
    pub title: PlainText,
    pub video_url: String,
    pub thumbnail_url: String,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl Video {
    fn validate(&self) -> Result<(), BuildError> {
        if self.title.text.len() > crate::blocks::MAX_VIDEO_TEXT_LEN {
            return Err(BuildError::message(format!(
                "video block title cannot exceed {} characters",
                crate::blocks::MAX_VIDEO_TEXT_LEN
            )));
        }
        if let Some(desc) = &self.description
            && desc.text.len() > crate::blocks::MAX_VIDEO_TEXT_LEN
        {
            return Err(BuildError::message(format!(
                "video block description cannot exceed {} characters",
                crate::blocks::MAX_VIDEO_TEXT_LEN
            )));
        }
        if self.alt_text.trim().is_empty() {
            return Err(BuildError::message(
                "video block alt_text must not be empty",
            ));
        }
        if !is_https(&self.video_url) {
            return Err(BuildError::message(
                "video block video_url must use https scheme",
            ));
        }
        if !is_https(&self.thumbnail_url) {
            return Err(BuildError::message(
                "video block thumbnail_url must use https scheme",
            ));
        }
        if let Some(title_url) = &self.title_url
            && !is_https(title_url)
        {
            return Err(BuildError::message(
                "video block title_url must use https scheme",
            ));
        }
        Ok(())
    }
}

