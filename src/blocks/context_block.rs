use serde::Serialize;

use crate::blocks::BuildError;
use crate::blocks::elements::ContextElement;

// https://docs.slack.dev/reference/block-kit/blocks/context-block/
#[slaq_macros::block(validate = Self::validate)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_requires_at_least_one_element() {
        let err = Context::new(Vec::new()).build().expect_err("empty context");
        assert_eq!(
            err,
            BuildError::message("context block requires at least one element")
        );
    }

    #[test]
    fn context_rejects_over_limit() {
        let mut items = Vec::new();
        for _ in 0..=crate::blocks::MAX_CONTEXT_ELEMENTS {
            // one over
            items.push(crate::blocks::ContextElement::plain_text("x"));
        }
        let err = Context::new(items).build().expect_err("too many");
        assert_eq!(
            err,
            BuildError::message(format!(
                "context block supports at most {} elements",
                crate::blocks::MAX_CONTEXT_ELEMENTS
            ))
        );
    }
}
