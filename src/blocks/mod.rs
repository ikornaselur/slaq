//! Typed builders for Slack Block Kit blocks.
//!
//! This module is expanding to cover every block type with ergonomic builders
//! and validations derived from Slack's documentation. 

pub mod elements;
pub mod rich_text;
pub mod text;
mod simple;
mod image_block;
mod file_block;
mod context_block;
mod context_actions_block;
mod actions_block;
mod section_block;
mod input_block;
mod video_block;
mod table_block;

pub use elements::{
    BlockElement, ButtonElement, ButtonStyle, ConfirmationDialog, ContextActionElement,
    ContextElement, FeedbackButton, FeedbackButtons, IconButton, SelectOption, StaticSelectElement,
};
pub use simple::{Divider, Header, Markdown};
pub use image_block::{Image, SlackFileRef};
pub use file_block::{File, FileSource};
pub use context_block::Context;
pub use context_actions_block::ContextActions;
pub use actions_block::Actions;
pub use section_block::Section;
pub use input_block::Input;
pub use video_block::Video;
pub use table_block::{Table, TableCell, ColumnSetting, ColumnAlignment};
pub use rich_text::{BroadcastRange, ListStyle, RichTextElement, RichTextNode, TextStyle};
pub use text::{MrkdwnText, PlainText, TextObject};

use serde::Serialize;
use thiserror::Error;

// https://docs.slack.dev/reference/block-kit/blocks/context-block/
pub(crate) const MAX_CONTEXT_ELEMENTS: usize = 10;
// https://docs.slack.dev/reference/block-kit/blocks/context-actions-block
pub(crate) const MAX_CONTEXT_ACTIONS_ELEMENTS: usize = 5;
// https://docs.slack.dev/reference/block-kit/blocks/actions-block
pub(crate) const MAX_ACTIONS_ELEMENTS: usize = 25;
// https://docs.slack.dev/reference/block-kit/blocks/section-block
pub(crate) const MAX_SECTION_FIELDS: usize = 10;
// https://docs.slack.dev/reference/block-kit/blocks/input-block
pub(crate) const MAX_INPUT_TEXT_LEN: usize = 2000;
// https://docs.slack.dev/reference/block-kit/blocks/video-block
pub(crate) const MAX_VIDEO_TEXT_LEN: usize = 200;
// https://docs.slack.dev/reference/block-kit/blocks/table-block
pub(crate) const MAX_TABLE_ROWS: usize = 100;
pub(crate) const MAX_TABLE_COLUMNS: usize = 20;

/// Errors returned when validating or building a block payload.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum BuildError {
    /// Generic failure with a user-facing message.
    #[error("{message}")]
    Message { message: String },
}

impl BuildError {
    /// Convenience constructor for `BuildError::Message`.
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message {
            message: message.into(),
        }
    }
}

pub type BuildResult = Result<Block, BuildError>;

 

/// Enumeration of all supported blocks.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Divider(Divider),
    Markdown(Markdown),
    Header(Header),
    Image(Image),
    File(File),
    Context(Context),
    ContextActions(ContextActions),
    Actions(Actions),
    Section(Section),
    Input(Input),
    Video(Video),
    RichText(RichText),
    Table(Table),
}

impl Block {
    /// Serialize this block into a `serde_json::Value`.
    ///
    /// # Panics
    /// If serialization fails (unexpected for well-formed blocks).
    #[must_use]
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("block serialization should never fail")
    }
}










#[slaq_macros::block(kind = "rich_text", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RichText {
    /// Nested rich text elements mirroring Slack's `rich_text` schema.
    pub elements: Vec<rich_text::RichTextElement>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

impl RichText {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "rich_text block requires at least one element",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_serializes_minimal() {
        let block = Divider::new().build().expect("divider build");
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"divider"}"#);
    }

    #[test]
    fn markdown_serializes_with_text_and_block_id() {
        let block = Markdown::new("hello")
            .block_id("block-1")
            .build()
            .expect("markdown build");
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"markdown\""));
        assert!(json.contains("\"text\":\"hello\""));
        assert!(json.contains("\"block_id\":\"block-1\""));
    }

    #[test]
    fn header_serializes_plain_text() {
        let block = Header::new(PlainText::new("Release"))
            .build()
            .expect("header build");
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"header\""));
        assert!(json.contains("\"text\":{\"type\":\"plain_text\""));
    }

    #[test]
    fn image_serializes_with_url() {
        let block = Image::new("Preview")
            .image_url("https://example.com/cat.png")
            .title(PlainText::new("cat"))
            .build()
            .expect("image build");
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"image\""));
        assert!(json.contains("\"image_url\":\"https://example.com/cat.png\""));
    }

    #[test]
    fn file_serializes_remote_source() {
        let block = File::new("ABCD1".to_string(), FileSource::Remote)
            .build()
            .expect("file build");
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"file\""));
        assert!(json.contains("\"external_id\":\"ABCD1\""));
        assert!(json.contains("\"source\":\"remote\""));
    }

    #[test]
    fn context_serializes_text_and_image() {
        let block = Context::new(vec![
            ContextElement::mrkdwn("*Location*: Dogpatch"),
            ContextElement::image(
                "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
                "pin",
            ),
        ])
        .build()
        .expect("context build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"context\""));
        assert!(json.contains("Dogpatch"));
        assert!(json.contains("red-drawing-pin"));
    }

    #[test]
    fn context_actions_serializes_feedback() {
        let positive = FeedbackButton::new("👍", "positive");
        let negative = FeedbackButton::new("👎", "negative");
        let block = ContextActions::new(vec![ContextActionElement::feedback(
            "feedback_1",
            positive,
            negative,
        )])
        .build()
        .expect("context actions build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"context_actions\""));
        assert!(json.contains("feedback_1"));
    }

    #[test]
    fn actions_serializes_button() {
        let button = ButtonElement::new(PlainText::new("Click"), "btn_1");
        let block = Actions::new(vec![BlockElement::from(button)])
            .build()
            .expect("actions build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"actions\""));
        assert!(json.contains("btn_1"));
    }

    #[test]
    fn actions_empty_fails() {
        let err = Actions::new(Vec::new())
            .build()
            .expect_err("expected error");
        assert!(err.to_string().contains("requires at least one element"));
    }

    #[test]
    fn section_serializes_with_text_and_accessory() {
        let section = Section::new()
            .text("*Deployment finished*")
            .accessory(BlockElement::from(ButtonElement::new(
                PlainText::new("Details"),
                "details_btn",
            )))
            .build()
            .expect("section build");

        let json = serde_json::to_string(&section).unwrap();
        assert!(json.contains("\"type\":\"section\""));
        assert!(json.contains("Deployment finished"));
        assert!(json.contains("details_btn"));
    }

    #[test]
    fn section_requires_content() {
        let err = Section::new().build().expect_err("expected section error");
        assert!(
            err.to_string()
                .contains("section block requires text or at least one field")
        );
    }

    #[test]
    fn input_serializes_with_text_input() {
        let element = BlockElement(serde_json::json!({
            "type": "plain_text_input",
            "action_id": "input_1"
        }));
        let block = Input::new(PlainText::new("Reason"), element)
            .hint(PlainText::new("Optional"))
            .build()
            .expect("input build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"input\""));
        assert!(json.contains("Reason"));
        assert!(json.contains("plain_text_input"));
    }

    #[test]
    fn input_dispatch_action_file_input_fails() {
        let element = BlockElement(serde_json::json!({
            "type": "file_input",
            "action_id": "files"
        }));
        let err = Input::new(PlainText::new("Upload"), element)
            .dispatch_action(true)
            .build()
            .expect_err("expected input error");
        assert!(
            err.to_string()
                .contains("input block cannot use dispatch_action with file_input")
        );
    }

    #[test]
    fn video_serializes_minimal() {
        let block = Video::new(
            PlainText::new("Slack demo"),
            "https://example.com/embed/123",
            "https://example.com/thumb.jpg",
            "Slack demo video",
        )
        .build()
        .expect("video build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"video\""));
        assert!(json.contains("Slack demo"));
        assert!(json.contains("thumb.jpg"));
    }

    #[test]
    fn video_rejects_non_https_urls() {
        let err = Video::new(
            PlainText::new("Slack demo"),
            "http://example.com/embed/123",
            "https://example.com/thumb.jpg",
            "Slack demo video",
        )
        .build()
        .expect_err("expected https validation");

        assert!(
            err.to_string()
                .contains("video block video_url must use https scheme")
        );
    }

    #[test]
    fn rich_text_serializes_section() {
        let section = rich_text::RichTextElement::section(vec![
            rich_text::RichTextNode::text("Hello"),
            rich_text::RichTextNode::styled_text("world", rich_text::TextStyle::new().bold()),
        ]);
        let block = RichText::new(vec![section])
            .build()
            .expect("rich text build");

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"type\":\"rich_text\""));
        assert!(json.contains("Hello"));
        assert!(json.contains("world"));
    }

    #[test]
    fn rich_text_requires_elements() {
        let err = RichText::new(Vec::<rich_text::RichTextElement>::new())
            .build()
            .expect_err("expected rich text error");
        assert!(
            err.to_string()
                .contains("rich_text block requires at least one element")
        );
    }

    #[test]
    fn table_serializes_raw_rows() {
        let rows = vec![
            vec![TableCell::raw("Header A"), TableCell::raw("Header B")],
            vec![TableCell::raw("Data 1A"), TableCell::raw("Data 1B")],
        ];
        let table = Table::new(rows)
            .column_settings(vec![
                ColumnSetting::new().align(ColumnAlignment::Left),
                ColumnSetting::new().wrap(true),
            ])
            .build()
            .expect("table build");

        let json = serde_json::to_string(&table).unwrap();
        assert!(json.contains("\"type\":\"table\""));
        assert!(json.contains("Header A"));
        assert!(json.contains("Data 1B"));
    }

    #[test]
    fn table_requires_rows() {
        let err = Table::new(Vec::<Vec<TableCell>>::new())
            .build()
            .expect_err("expected table error");
        assert!(
            err.to_string()
                .contains("table block requires at least one row")
        );
    }
}
