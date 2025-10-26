use serde::Serialize;

use crate::blocks::elements::BlockElement;
use crate::blocks::BuildError;

/// Actions block.
///
/// Wraps interactive elements like buttons and select menus.
///
/// Constraints:
/// - Must contain at least one element.
/// - Maximum of `MAX_ACTIONS_ELEMENTS` elements per block.
/// - For `static_select` and `multi_static_select` items, exactly one of
///   `options` or `option_groups` must be provided.
/// - If provided, `max_selected_items` on `multi_static_select` must be > 0.
///
/// See: https://docs.slack.dev/reference/block-kit/blocks/actions-block
#[slaq_macros::block(kind = "actions", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Actions {
    /// Interactive elements displayed horizontally.
    pub elements: Vec<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional block identifier.
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
        // Validate static select invariants for contained elements
        for el in &self.elements {
            let ty = el.0.get("type").and_then(|v| v.as_str()).unwrap_or("");
            match ty {
                "static_select" => {
                    validate_static_select(&el.0, false)?;
                }
                "multi_static_select" => {
                    validate_static_select(&el.0, true)?;
                    // Optional: if max_selected_items present, ensure > 0
                    if let Some(msi) = el.0.get("max_selected_items").and_then(|v| v.as_u64()) {
                        if msi == 0 {
                            return Err(BuildError::message(
                                "multi_static_select max_selected_items must be greater than 0",
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn validate_static_select(value: &serde_json::Value, _is_multi: bool) -> Result<(), BuildError> {
    let has_options = value
        .get("options")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    let has_groups = value
        .get("option_groups")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    if has_options && has_groups {
        return Err(BuildError::message(
            "static_select cannot specify both options and option_groups",
        ));
    }
    if !has_options && !has_groups {
        return Err(BuildError::message(
            "static_select requires options or option_groups",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{elements::StaticSelectElement, PlainText};
    use crate::blocks::elements::OptionGroup;

    #[test]
    fn static_select_requires_options_or_groups() {
        let sel = StaticSelectElement::new("s");
        let err = Actions::new(vec![BlockElement::from(sel)])
            .build()
            .expect_err("expected validation error");
        assert_eq!(
            err,
            BuildError::message("static_select requires options or option_groups")
        );
    }

    #[test]
    fn static_select_cannot_have_both() {
        let sel = StaticSelectElement::new("s")
            .options(vec![crate::blocks::SelectOption::new(PlainText::new("A"), "a")])
            .option_groups(vec![OptionGroup::new(
                PlainText::new("G"),
                vec![crate::blocks::SelectOption::new(PlainText::new("A"), "a")],
            )]);
        let err = Actions::new(vec![BlockElement::from(sel)])
            .build()
            .expect_err("expected validation error");
        assert_eq!(
            err,
            BuildError::message("static_select cannot specify both options and option_groups")
        );
    }
}
