use serde::Serialize;

use crate::blocks::elements::ContextActionElement;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/context-actions-block
#[slaq_macros::block(kind = "context_actions", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContextActions {
    pub elements: Vec<ContextActionElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl ContextActions {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "context_actions block requires at least one element",
            ));
        }
        if self.elements.len() > crate::blocks::MAX_CONTEXT_ACTIONS_ELEMENTS {
            return Err(BuildError::message(format!(
                "context_actions block supports at most {} elements",
                crate::blocks::MAX_CONTEXT_ACTIONS_ELEMENTS
            )));
        }
        Ok(())
    }
}

