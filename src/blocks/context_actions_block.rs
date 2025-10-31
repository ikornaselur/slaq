use serde::Serialize;

use crate::blocks::BuildError;
use crate::blocks::elements::ContextActionElement;

// https://docs.slack.dev/reference/block-kit/blocks/context-actions-block
#[slaq_macros::block(validate = Self::validate)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_actions_requires_at_least_one_element() {
        let err = ContextActions::new(Vec::new())
            .build()
            .expect_err("empty context_actions");
        assert_eq!(
            err,
            BuildError::message("context_actions block requires at least one element")
        );
    }

    #[test]
    fn context_actions_rejects_over_limit() {
        // Use feedback buttons element for minimal JSON
        let pos = crate::blocks::FeedbackButton::new("Yes", "y");
        let neg = crate::blocks::FeedbackButton::new("No", "n");
        let mut items = Vec::new();
        for _ in 0..=crate::blocks::MAX_CONTEXT_ACTIONS_ELEMENTS {
            // one over
            items.push(crate::blocks::ContextActionElement::feedback(
                "fb",
                pos.clone(),
                neg.clone(),
            ));
        }
        let err = ContextActions::new(items).build().expect_err("too many");
        assert_eq!(
            err,
            BuildError::message(format!(
                "context_actions block supports at most {} elements",
                crate::blocks::MAX_CONTEXT_ACTIONS_ELEMENTS
            ))
        );
    }

    #[test]
    fn context_actions_icon_button_serializes() {
        let icon_el = crate::blocks::ContextActionElement::icon(
            ":smile:",
            crate::blocks::PlainText::new("Great"),
            "act",
        );
        let block = ContextActions::new(vec![icon_el]).build().expect("build");
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"context_actions\""));
        assert!(json.contains("\"type\":\"icon_button\""));
        assert!(json.contains("Great"));
    }
}
