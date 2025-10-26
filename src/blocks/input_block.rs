use serde::Serialize;

use crate::blocks::BuildError;
use crate::blocks::elements::BlockElement;
use crate::blocks::text::PlainText;

/// Input block.
///
/// Captures user input via an element (e.g., `plain_text_input`, select, datepicker).
///
/// Constraints:
/// - `label` and optional `hint` are limited to `MAX_INPUT_TEXT_LEN` characters.
/// - `dispatch_action` is not supported with the `file_input` element.
///
/// See: <https://docs.slack.dev/reference/block-kit/blocks/input-block>
#[slaq_macros::block(validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    /// User-facing label for the input.
    pub label: PlainText,
    /// The interactive element to render.
    pub element: BlockElement,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Trigger actions as a user types.
    pub dispatch_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Helper text displayed under the input.
    pub hint: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Whether the input is optional.
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional block identifier.
    pub block_id: Option<String>,
}

impl Input {
    fn validate(&self) -> Result<(), BuildError> {
        // If element is file_input, dispatch_action is unsupported
        if let Some(true) = self.dispatch_action {
            let ty = self
                .element
                .0
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            if ty == "file_input" {
                return Err(BuildError::message(
                    "input block cannot use dispatch_action with file_input",
                ));
            }
        }
        // Label length per Slack: 2000 max
        if self.label.text.len() > crate::blocks::MAX_INPUT_TEXT_LEN {
            return Err(BuildError::message(format!(
                "input block label cannot exceed {} characters",
                crate::blocks::MAX_INPUT_TEXT_LEN
            )));
        }
        if let Some(hint) = &self.hint
            && hint.text.len() > crate::blocks::MAX_INPUT_TEXT_LEN
        {
            return Err(BuildError::message(format!(
                "input block hint cannot exceed {} characters",
                crate::blocks::MAX_INPUT_TEXT_LEN
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_over_limit_rejected() {
        let long = "x".repeat(crate::blocks::MAX_INPUT_TEXT_LEN + 1);
        let element = BlockElement(serde_json::json!({
            "type": "plain_text_input",
            "action_id": "a"
        }));
        let err = Input::new(PlainText::new(long), element)
            .build()
            .expect_err("expected label length error");
        assert_eq!(
            err,
            BuildError::message(format!(
                "input block label cannot exceed {} characters",
                crate::blocks::MAX_INPUT_TEXT_LEN
            ))
        );
    }

    #[test]
    fn hint_over_limit_rejected() {
        let long = "x".repeat(crate::blocks::MAX_INPUT_TEXT_LEN + 1);
        let element = BlockElement(serde_json::json!({
            "type": "plain_text_input",
            "action_id": "a"
        }));
        let err = Input::new(PlainText::new("Label"), element)
            .hint(PlainText::new(long))
            .build()
            .expect_err("expected hint length error");
        assert_eq!(
            err,
            BuildError::message(format!(
                "input block hint cannot exceed {} characters",
                crate::blocks::MAX_INPUT_TEXT_LEN
            ))
        );
    }
}
