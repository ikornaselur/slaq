use serde::Serialize;

use crate::blocks::elements::BlockElement;
use crate::blocks::text::PlainText;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/input-block
#[slaq_macros::block(kind = "input", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub label: PlainText,
    pub element: BlockElement,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dispatch_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hint: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
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
            && hint.text.len() > crate::blocks::MAX_INPUT_TEXT_LEN {
                return Err(BuildError::message(format!(
                    "input block hint cannot exceed {} characters",
                    crate::blocks::MAX_INPUT_TEXT_LEN
                )));
            }
        Ok(())
    }
}

