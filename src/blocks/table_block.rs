use serde::Serialize;

use crate::blocks::rich_text;
use crate::blocks::BuildError;

// https://docs.slack.dev/reference/block-kit/blocks/table-block
#[slaq_macros::block(kind = "table", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub rows: Vec<Vec<TableCell>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub column_settings: Option<Vec<ColumnSetting>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TableCell {
    RawText { text: String },
    RichText { elements: Vec<rich_text::RichTextElement> },
}

impl TableCell {
    #[must_use]
    pub fn raw(text: impl Into<String>) -> Self {
        TableCell::RawText { text: text.into() }
    }
    #[must_use]
    pub fn rich(elements: impl Into<Vec<rich_text::RichTextElement>>) -> Self {
        TableCell::RichText { elements: elements.into() }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ColumnSetting {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub align: Option<ColumnAlignment>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_wrapped: Option<bool>,
}

impl Default for ColumnSetting {
    fn default() -> Self { Self::new() }
}

impl ColumnSetting {
    #[must_use]
    pub fn new() -> Self { Self { align: None, is_wrapped: None } }
    #[must_use]
    pub fn align(mut self, value: ColumnAlignment) -> Self { self.align = Some(value); self }
    #[must_use]
    pub fn wrap(mut self, value: bool) -> Self { self.is_wrapped = Some(value); self }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlignment { Left, Center, Right }

impl Table {
    fn validate(&self) -> Result<(), BuildError> {
        if self.rows.is_empty() {
            return Err(BuildError::message("table block requires at least one row"));
        }
        if self.rows.len() > crate::blocks::MAX_TABLE_ROWS {
            return Err(BuildError::message(format!(
                "table block supports at most {} rows",
                crate::blocks::MAX_TABLE_ROWS
            )));
        }
        for (idx, row) in self.rows.iter().enumerate() {
            if row.is_empty() {
                return Err(BuildError::message(format!(
                    "table block row {idx} must contain at least one cell"
                )));
            }
            if row.len() > crate::blocks::MAX_TABLE_COLUMNS {
                return Err(BuildError::message(format!(
                    "table block rows support at most {} cells",
                    crate::blocks::MAX_TABLE_COLUMNS
                )));
            }
        }
        if let Some(settings) = &self.column_settings
            && settings.len() > crate::blocks::MAX_TABLE_COLUMNS
        {
            return Err(BuildError::message(format!(
                "table block column_settings supports at most {} entries",
                crate::blocks::MAX_TABLE_COLUMNS
            )));
        }
        Ok(())
    }
}

