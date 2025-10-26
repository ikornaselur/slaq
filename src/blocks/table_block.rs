use serde::Serialize;

use crate::blocks::rich_text;
use crate::blocks::BuildError;

/// Table block.
///
/// Displays rows of cells with optional per-column settings.
///
/// Constraints:
/// - Requires at least one row, each row at least one cell.
/// - At most `MAX_TABLE_ROWS` rows and `MAX_TABLE_COLUMNS` columns per row.
/// - `column_settings`, if present, may have at most `MAX_TABLE_COLUMNS` entries.
///
/// See: https://docs.slack.dev/reference/block-kit/blocks/table-block
#[slaq_macros::block(kind = "table", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Table {
    /// Rows of table cells.
    pub rows: Vec<Vec<TableCell>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional per-column display settings.
    pub column_settings: Option<Vec<ColumnSetting>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Optional block identifier.
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TableCell {
    /// A raw string cell.
    RawText { text: String },
    /// A rich text cell composed of rich text elements.
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
    /// Horizontal alignment.
    pub align: Option<ColumnAlignment>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Whether text in this column wraps.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_rejects_too_many_rows() {
        let rows = vec![vec![TableCell::raw("x")]; crate::blocks::MAX_TABLE_ROWS + 1];
        let res = Table::new(rows).build();
        assert!(res.is_err());
        let msg = format!(
            "table block supports at most {} rows",
            crate::blocks::MAX_TABLE_ROWS
        );
        assert_eq!(res.unwrap_err(), BuildError::message(msg));
    }

    #[test]
    fn table_rejects_too_many_columns() {
        let row = vec![TableCell::raw("x"); crate::blocks::MAX_TABLE_COLUMNS + 1];
        let rows = vec![row];
        let res = Table::new(rows).build();
        assert!(res.is_err());
        let msg = format!(
            "table block rows support at most {} cells",
            crate::blocks::MAX_TABLE_COLUMNS
        );
        assert_eq!(res.unwrap_err(), BuildError::message(msg));
    }
}
