use serde::Serialize;

use crate::blocks::elements::BlockElement;
use crate::blocks::text::TextObject;
use crate::blocks::BuildError;

/// Section block.
///
/// Rich text or fields content with an optional accessory element.
///
/// Constraints:
/// - Must provide `text` or at least one `fields` entry.
/// - At most `MAX_SECTION_FIELDS` fields.
/// - If the accessory is a `static_select` or `multi_static_select`,
///   exactly one of `options` or `option_groups` must be present.
///
/// See: https://docs.slack.dev/reference/block-kit/blocks/section-block
#[slaq_macros::block(kind = "section", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// The main text content (commonly mrkdwn).
    pub text: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Up to `MAX_SECTION_FIELDS` text objects.
    pub fields: Option<Vec<TextObject>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional accessory (e.g., button or select menu).
    pub accessory: Option<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional expand flag per Slack docs.
    pub expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional block identifier.
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
        // If an accessory is a static select, validate options/groups invariants
        if let Some(acc) = &self.accessory {
            let ty = acc.0.get("type").and_then(|v| v.as_str()).unwrap_or("");
            match ty {
                "static_select" => validate_static_select(&acc.0, false)?,
                "multi_static_select" => validate_static_select(&acc.0, true)?,
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
    use crate::blocks::elements::StaticSelectElement;
    use crate::blocks::MrkdwnText;

    #[test]
    fn section_accessory_select_requires_content() {
        let sel = StaticSelectElement::new("id");
        let err = Section::new()
            .text(MrkdwnText::new("t"))
            .accessory(sel)
            .build()
            .expect_err("error");
        assert_eq!(
            err,
            BuildError::message("static_select requires options or option_groups")
        );
    }
}
