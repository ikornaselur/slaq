use serde::Serialize;

use crate::blocks::elements::BlockElement;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/actions-block
#[slaq_macros::block(kind = "actions", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Actions {
    pub elements: Vec<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl Actions {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "actions block requires at least one element",
            ));
        }
        if self.elements.len() > crate::blocks::MAX_ACTIONS_ELEMENTS {
            return Err(BuildError::message(format!(
                "actions block supports at most {} elements",
                crate::blocks::MAX_ACTIONS_ELEMENTS
            )));
        }
        Ok(())
    }
}

