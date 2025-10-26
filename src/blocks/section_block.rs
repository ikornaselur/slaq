use serde::Serialize;

use crate::blocks::elements::BlockElement;
use crate::blocks::text::TextObject;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/section-block
#[slaq_macros::block(kind = "section", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<Vec<TextObject>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub accessory: Option<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl Section {
    fn validate(&self) -> Result<(), BuildError> {
        let has_text = self.text.is_some();
        let has_fields = self
            .fields
            .as_ref()
            .is_some_and(|v| !v.is_empty());
        if !has_text && !has_fields {
            return Err(BuildError::message(
                "section block requires text or at least one field",
            ));
        }
        if let Some(fields) = &self.fields
            && fields.len() > crate::blocks::MAX_SECTION_FIELDS
        {
            return Err(BuildError::message(format!(
                "section fields can contain at most {} items",
                crate::blocks::MAX_SECTION_FIELDS
            )));
        }
        Ok(())
    }
}

