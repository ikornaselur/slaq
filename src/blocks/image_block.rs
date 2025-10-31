use serde::Serialize;

use crate::blocks::BuildError;
use crate::blocks::text::PlainText;

#[slaq_macros::block(validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Image {
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub slack_file: Option<SlackFileRef>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SlackFileRef {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

impl Image {
    fn validate(&self) -> Result<(), BuildError> {
        if self.alt_text.len() > crate::blocks::MAX_IMAGE_ALT_TEXT_LEN {
            return Err(BuildError::message(format!(
                "image block alt_text cannot exceed {} characters",
                crate::blocks::MAX_IMAGE_ALT_TEXT_LEN
            )));
        }
        match (self.image_url.as_ref(), self.slack_file.as_ref()) {
            (None, None) => {
                return Err(BuildError::message(
                    "image block requires either image_url or slack_file",
                ));
            }
            (Some(_), Some(_)) => {
                return Err(BuildError::message(
                    "image block must choose either image_url or slack_file, not both",
                ));
            }
            (_, Some(file_ref)) => {
                if file_ref.url.is_none() && file_ref.id.is_none() {
                    return Err(BuildError::message(
                        "slack_file reference requires either url or id",
                    ));
                }
            }
            _ => {}
        }
        if let Some(url) = &self.image_url
            && url.len() > crate::blocks::MAX_IMAGE_URL_LEN
        {
            return Err(BuildError::message(format!(
                "image block image_url cannot exceed {} characters",
                crate::blocks::MAX_IMAGE_URL_LEN
            )));
        }
        if let Some(title) = &self.title
            && title.text.len() > crate::blocks::MAX_IMAGE_TITLE_TEXT_LEN
        {
            return Err(BuildError::message(format!(
                "image block title text cannot exceed {} characters",
                crate::blocks::MAX_IMAGE_TITLE_TEXT_LEN
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_requires_one_source() {
        let err = Image::new("alt").build().expect_err("missing source");
        assert_eq!(
            err,
            BuildError::message("image block requires either image_url or slack_file")
        );
    }

    #[test]
    fn image_cannot_have_both_sources() {
        let err = Image::new("alt")
            .image_url("https://example")
            .slack_file(SlackFileRef {
                url: Some("https://files".into()),
                id: None,
            })
            .build()
            .expect_err("both sources");
        assert_eq!(
            err,
            BuildError::message("image block must choose either image_url or slack_file, not both")
        );
    }
}
