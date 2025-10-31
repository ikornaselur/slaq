use serde::Serialize;

use crate::blocks::BuildError;
use crate::blocks::elements::BlockElement;
use crate::blocks::text::TextObject;

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
/// See: <https://docs.slack.dev/reference/block-kit/blocks/section-block>
#[slaq_macros::block(validate = Self::validate)]
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
        let has_fields = self.fields.as_ref().is_some_and(|v| !v.is_empty());
        if !has_text && !has_fields {
            return Err(BuildError::message(
                "section block requires text or at least one field",
            ));
        }
        if let Some(text) = &self.text {
            let len = match text {
                crate::blocks::TextObject::Plain(p) => p.text.len(),
                crate::blocks::TextObject::Mrkdwn(m) => m.text.len(),
            };
            if len == 0 || len > crate::blocks::MAX_SECTION_TEXT_LEN {
                return Err(BuildError::message(format!(
                    "section text length must be 1..={} characters",
                    crate::blocks::MAX_SECTION_TEXT_LEN
                )));
            }
        }
        if let Some(fields) = &self.fields
            && fields.len() > crate::blocks::MAX_SECTION_FIELDS
        {
            return Err(BuildError::message(format!(
                "section fields can contain at most {} items",
                crate::blocks::MAX_SECTION_FIELDS
            )));
        }
        if let Some(fields) = &self.fields {
            for (idx, item) in fields.iter().enumerate() {
                let len = match item {
                    crate::blocks::TextObject::Plain(p) => p.text.len(),
                    crate::blocks::TextObject::Mrkdwn(m) => m.text.len(),
                };
                if len > crate::blocks::MAX_SECTION_FIELD_TEXT_LEN {
                    return Err(BuildError::message(format!(
                        "section field {} text cannot exceed {} characters",
                        idx,
                        crate::blocks::MAX_SECTION_FIELD_TEXT_LEN
                    )));
                }
            }
        }
        // If an accessory is a static select, validate options/groups invariants
        if let Some(acc) = &self.accessory {
            let ty = acc.0.get("type").and_then(|v| v.as_str()).unwrap_or("");
            match ty {
                "static_select" => validate_static_select(&acc.0, false)?,
                "multi_static_select" => {
                    validate_static_select(&acc.0, true)?;
                    if let Some(msi) = acc
                        .0
                        .get("max_selected_items")
                        .and_then(serde_json::Value::as_u64)
                        && msi == 0
                    {
                        return Err(BuildError::message(
                            "multi_static_select max_selected_items must be greater than 0",
                        ));
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
        .and_then(serde_json::Value::as_array)
        .is_some_and(|a| !a.is_empty());
    let has_groups = value
        .get("option_groups")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|a| !a.is_empty());
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
    use crate::blocks::MrkdwnText;
    use crate::blocks::elements::StaticSelectElement;

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

    #[test]
    fn section_fields_over_limit_rejected() {
        let mut fields = Vec::new();
        for _ in 0..=crate::blocks::MAX_SECTION_FIELDS {
            // one over
            fields.push(MrkdwnText::new("f").into());
        }
        let err = Section::new().fields(fields).build().expect_err("error");
        assert_eq!(
            err,
            BuildError::message(format!(
                "section fields can contain at most {} items",
                crate::blocks::MAX_SECTION_FIELDS
            ))
        );
    }
}
