use serde::Serialize;

use crate::blocks::text::PlainText;
use crate::blocks::BuildError;

#[slaq_macros::block(kind = "image", validate = Self::validate)]
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

        Ok(())
    }
}

