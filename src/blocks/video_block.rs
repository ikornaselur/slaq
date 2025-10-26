use serde::Serialize;

use crate::blocks::text::PlainText;
use crate::blocks::BuildError;

fn is_https(url: &str) -> bool { url.starts_with("https://") }

/// Video block.
///
/// Embeds a video with title and preview thumbnail.
///
/// Constraints:
/// - `video_url`, `thumbnail_url`, and optional `title_url` must use HTTPS.
/// - `alt_text` must not be empty.
/// - `title` and `description` (if provided) are limited to `MAX_VIDEO_TEXT_LEN` characters.
///
/// See: https://docs.slack.dev/reference/block-kit/blocks/video-block
#[slaq_macros::block(kind = "video", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Video {
    /// Title displayed above the video.
    pub title: PlainText,
    pub video_url: String,
    pub thumbnail_url: String,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional description below the title.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Video {
        Video::new(
            PlainText::new("Title"),
            "https://example.com/video",
            "https://example.com/thumb",
            "alt",
        )
    }

    #[test]
    fn rejects_non_https_urls() {
        let v = Video::new(
            PlainText::new("Title"),
            "http://example.com/video",
            "https://example.com/thumb",
            "alt",
        );
        assert_eq!(
            v.build().unwrap_err(),
            BuildError::message("video block video_url must use https scheme")
        );

        let v = Video::new(
            PlainText::new("Title"),
            "https://example.com/video",
            "http://example.com/thumb",
            "alt",
        );
        assert_eq!(
            v.build().unwrap_err(),
            BuildError::message("video block thumbnail_url must use https scheme")
        );

        let v = base().title_url("http://example.com/title");
        assert_eq!(
            v.build().unwrap_err(),
            BuildError::message("video block title_url must use https scheme")
        );
    }

    #[test]
    fn rejects_empty_alt_text() {
        let v = Video::new(
            PlainText::new("Title"),
            "https://example.com/video",
            "https://example.com/thumb",
            "   ",
        );
        assert_eq!(
            v.build().unwrap_err(),
            BuildError::message("video block alt_text must not be empty")
        );
    }
}
