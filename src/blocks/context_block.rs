use serde::Serialize;

use crate::blocks::elements::ContextElement;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/context-block/
#[slaq_macros::block(kind = "context", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub elements: Vec<ContextElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl Context {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "context block requires at least one element",
            ));
        }
        if self.elements.len() > crate::blocks::MAX_CONTEXT_ELEMENTS {
            return Err(BuildError::message(format!(
                "context block supports at most {} elements",
                crate::blocks::MAX_CONTEXT_ELEMENTS
            )));
        }
        Ok(())
    }
}

