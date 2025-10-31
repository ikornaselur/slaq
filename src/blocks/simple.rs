use crate::blocks::BuildError;
use serde::Serialize;

use crate::blocks::text::PlainText;

#[slaq_macros::block()]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Divider {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block()]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Markdown {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub text: PlainText,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl Header {
    fn validate(&self) -> Result<(), BuildError> {
        if self.text.text.len() > crate::blocks::MAX_HEADER_TEXT_LEN {
            return Err(BuildError::message(format!(
                "header block text cannot exceed {} characters",
                crate::blocks::MAX_HEADER_TEXT_LEN
            )));
        }
        Ok(())
    }
}
