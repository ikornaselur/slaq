//! Typed builders for Slack Block Kit blocks.
//!
//! This module is expanding to cover every block type with ergonomic builders
//! and validations derived from Slack's documentation. 

pub mod elements;
pub mod rich_text;
pub mod text;

pub use elements::{
    BlockElement, ButtonElement, ButtonStyle, ConfirmationDialog, ContextActionElement,
    ContextElement, FeedbackButton, FeedbackButtons, IconButton, SelectOption, StaticSelectElement,
};
pub use rich_text::{BroadcastRange, ListStyle, RichTextElement, RichTextNode, TextStyle};
pub use text::{MrkdwnText, PlainText, TextObject};

use serde::Serialize;
use thiserror::Error;

// https://docs.slack.dev/reference/block-kit/blocks/context-block/
const MAX_CONTEXT_ELEMENTS: usize = 10;
// https://docs.slack.dev/reference/block-kit/blocks/context-actions-block
const MAX_CONTEXT_ACTIONS_ELEMENTS: usize = 5;
// https://docs.slack.dev/reference/block-kit/blocks/actions-block
const MAX_ACTIONS_ELEMENTS: usize = 25;
// https://docs.slack.dev/reference/block-kit/blocks/section-block
const MAX_SECTION_FIELDS: usize = 10;
// https://docs.slack.dev/reference/block-kit/blocks/input-block
const MAX_INPUT_TEXT_LEN: usize = 2000;
// https://docs.slack.dev/reference/block-kit/blocks/video-block
const MAX_VIDEO_TEXT_LEN: usize = 200;
// https://docs.slack.dev/reference/block-kit/blocks/table-block
const MAX_TABLE_ROWS: usize = 100;
const MAX_TABLE_COLUMNS: usize = 20;

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

fn is_https(url: &str) -> bool {
    url.starts_with("https://")
}

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
    #[must_use]
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("block serialization should never fail")
    }
}

#[slaq_macros::block(kind = "divider")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Divider {
    /// A unique identifier for a block. If not specified, one will be generated.
    /// Maximum length for this field is 255 characters. `block_id` should be unique
    /// for each message iteration.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "markdown")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Markdown {
    /// The standard markdown-formatted text. Limit 12,000 characters max.
    pub text: String,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "header")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Header {
    /// The text for the block, in the form of a plain-text object. 150 chars max per Slack docs.
    pub text: PlainText,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "image", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Image {
    /// A tooltip for the image. Required for accessibility.
    pub alt_text: String,
    /// The URL for a publicly hosted image. One of `image_url` or `slack_file` must be provided.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,
    /// A Slack image file reference. Optional and mutually exclusive with `image_url`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub slack_file: Option<SlackFileRef>,
    /// Optional title for the image (plain_text only).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<PlainText>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SlackFileRef {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
}

#[slaq_macros::block(kind = "file")]
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct File {
    /// The external unique ID for this file.
    pub external_id: String,
    /// The source for the file (currently always `remote`).
    pub source: FileSource,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileSource {
    Remote,
}

#[slaq_macros::block(kind = "context", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Context {
    /// An array of text objects and/or images, up to 10 entries.
    pub elements: Vec<ContextElement>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "context_actions", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContextActions {
    /// Contextual action elements (feedback buttons or icon buttons). Max 5.
    pub elements: Vec<ContextActionElement>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "actions", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Actions {
    /// Interactive elements such as buttons or select menus. Max 25.
    pub elements: Vec<BlockElement>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "section", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// Primary text content for the section (mrkdwn by default, plain_text allowed).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<TextObject>,
    /// Compact field list; required when `text` is absent. Max 10 items.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<Vec<TextObject>>,
    /// Optional accessory element such as a button, select, or image.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub accessory: Option<BlockElement>,
    /// Controls whether the section auto-expands (used by AI surfaces).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expand: Option<bool>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "input", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    /// Label shown above the input element (plain_text only, <= 2000 chars).
    pub label: PlainText,
    /// The interactive element to render (text input, select, etc.).
    pub element: BlockElement,
    /// Dispatches block_actions payloads on user interaction. Unsupported for file_input.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dispatch_action: Option<bool>,
    /// Optional helper text displayed below the input.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hint: Option<PlainText>,
    /// Whether the field may be left empty on submission.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub optional: Option<bool>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "video", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Video {
    /// Video title in the form of a plain-text object (must be < 200 characters).
    pub title: PlainText,
    /// HTTPS URL of the embeddable video.
    pub video_url: String,
    /// HTTPS thumbnail image URL to display for the video.
    pub thumbnail_url: String,
    /// Tooltip for the video, required for accessibility.
    pub alt_text: String,
    /// Optional description (plain_text, < 200 characters).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<PlainText>,
    /// Hyperlink for the title text (must be HTTPS).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title_url: Option<String>,
    /// Provider icon URL, such as a service logo.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider_icon_url: Option<String>,
    /// Provider name, e.g., YouTube.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider_name: Option<String>,
    /// Author name to display (< 50 characters).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author_name: Option<String>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "rich_text", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RichText {
    /// Nested rich text elements mirroring Slack's rich_text schema.
    pub elements: Vec<rich_text::RichTextElement>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "table", validate = Self::validate)]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Table {
    /// Rows within the table. Maximum of 100 rows.
    pub rows: Vec<Vec<TableCell>>,
    /// Optional column settings (alignment and wrapping), up to 20 columns.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub column_settings: Option<Vec<ColumnSetting>>,
    /// Optional block identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TableCell {
    RawText {
        text: String,
    },
    RichText {
        elements: Vec<rich_text::RichTextElement>,
    },
}

impl TableCell {
    #[must_use]
    pub fn raw(text: impl Into<String>) -> Self {
        TableCell::RawText { text: text.into() }
    }

    #[must_use]
    pub fn rich(elements: impl Into<Vec<rich_text::RichTextElement>>) -> Self {
        TableCell::RichText {
            elements: elements.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ColumnSetting {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub align: Option<ColumnAlignment>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_wrapped: Option<bool>,
}

impl ColumnSetting {
    #[must_use]
    pub fn new() -> Self {
        Self {
            align: None,
            is_wrapped: None,
        }
    }

    #[must_use]
    pub fn align(mut self, value: ColumnAlignment) -> Self {
        self.align = Some(value);
        self
    }

    #[must_use]
    pub fn wrap(mut self, value: bool) -> Self {
        self.is_wrapped = Some(value);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

impl Image {
    fn validate(&self) -> Result<(), BuildError> {
        match (self.image_url.as_ref(), self.slack_file.as_ref()) {
            (None, None) => {
                return Err(BuildError::message(
                    "image block requires either image_url or slack_file",
                ));
            }
            (Some(_), Some(_)) => {
                return Err(BuildError::message(
                    "image block must choose either image_url or slack_file, not both",
                ));
            }
            (_, Some(file_ref)) => {
                if file_ref.url.is_none() && file_ref.id.is_none() {
                    return Err(BuildError::message(
                        "slack_file reference requires either url or id",
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Context {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "context block requires at least one element",
            ));
        }
        if self.elements.len() > MAX_CONTEXT_ELEMENTS {
            return Err(BuildError::message(format!(
                "context block supports at most {} elements",
                MAX_CONTEXT_ELEMENTS
            )));
        }
        Ok(())
    }
}

impl ContextActions {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "context_actions block requires at least one element",
            ));
        }
        if self.elements.len() > MAX_CONTEXT_ACTIONS_ELEMENTS {
            return Err(BuildError::message(format!(
                "context_actions block supports at most {} elements",
                MAX_CONTEXT_ACTIONS_ELEMENTS
            )));
        }
        Ok(())
    }
}

impl Actions {
    fn validate(&self) -> Result<(), BuildError> {
        if self.elements.is_empty() {
            return Err(BuildError::message(
                "actions block requires at least one element",
            ));
        }
        if self.elements.len() > MAX_ACTIONS_ELEMENTS {
            return Err(BuildError::message(format!(
                "actions block supports at most {} elements",
                MAX_ACTIONS_ELEMENTS
            )));
        }
        Ok(())
    }
}

impl Section {
    fn validate(&self) -> Result<(), BuildError> {
        let has_text = self.text.is_some();
        let field_count = self.fields.as_ref().map(|f| f.len()).unwrap_or(0);

        if !has_text && field_count == 0 {
            return Err(BuildError::message(
                "section block requires text or at least one field",
            ));
        }
        if let Some(fields) = &self.fields {
            if fields.is_empty() {
                return Err(BuildError::message(
                    "section block fields must contain at least one entry",
                ));
            }
            if fields.len() > MAX_SECTION_FIELDS {
                return Err(BuildError::message(format!(
                    "section block supports at most {} fields",
                    MAX_SECTION_FIELDS
                )));
            }
        }
        Ok(())
    }
}

impl Input {
    fn validate(&self) -> Result<(), BuildError> {
        if self.label.text.is_empty() {
            return Err(BuildError::message("input block label must not be empty"));
        }
        if self.label.text.len() > MAX_INPUT_TEXT_LEN {
            return Err(BuildError::message(format!(
                "input block label cannot exceed {} characters",
                MAX_INPUT_TEXT_LEN
            )));
        }
        if let Some(hint) = &self.hint {
            if hint.text.len() > MAX_INPUT_TEXT_LEN {
                return Err(BuildError::message(format!(
                    "input block hint cannot exceed {} characters",
                    MAX_INPUT_TEXT_LEN
                )));
            }
        }
        if self.dispatch_action.unwrap_or(false) {
            if let Some(element_type) = self.element.0.get("type").and_then(|v| v.as_str()) {
                if element_type == "file_input" {
                    return Err(BuildError::message(
                        "input block cannot use dispatch_action with file_input element",
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Video {
    fn validate(&self) -> Result<(), BuildError> {
        if self.title.text.is_empty() {
            return Err(BuildError::message("video block title must not be empty"));
        }
        if self.title.text.len() > MAX_VIDEO_TEXT_LEN {
            return Err(BuildError::message(format!(
                "video block title cannot exceed {} characters",
                MAX_VIDEO_TEXT_LEN
            )));
        }
        if let Some(description) = &self.description {
            if description.text.len() > MAX_VIDEO_TEXT_LEN {
                return Err(BuildError::message(format!(
                    "video block description cannot exceed {} characters",
                    MAX_VIDEO_TEXT_LEN
                )));
            }
        }
        if self.alt_text.trim().is_empty() {
            return Err(BuildError::message(
                "video block alt_text must not be empty",
            ));
        }
        if !is_https(&self.video_url) {
            return Err(BuildError::message(
                "video block video_url must use https scheme",
            ));
        }
        if !is_https(&self.thumbnail_url) {
            return Err(BuildError::message(
                "video block thumbnail_url must use https scheme",
            ));
        }
        if let Some(title_url) = &self.title_url {
            if !is_https(title_url) {
                return Err(BuildError::message(
                    "video block title_url must use https scheme",
                ));
            }
        }
        Ok(())
    }
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

impl Table {
    fn validate(&self) -> Result<(), BuildError> {
        if self.rows.is_empty() {
            return Err(BuildError::message("table block requires at least one row"));
        }
        if self.rows.len() > MAX_TABLE_ROWS {
            return Err(BuildError::message(format!(
                "table block supports at most {} rows",
                MAX_TABLE_ROWS
            )));
        }
        for (idx, row) in self.rows.iter().enumerate() {
            if row.is_empty() {
                return Err(BuildError::message(format!(
                    "table block row {} must contain at least one cell",
                    idx
                )));
            }
            if row.len() > MAX_TABLE_COLUMNS {
                return Err(BuildError::message(format!(
                    "table block rows support at most {} cells",
                    MAX_TABLE_COLUMNS
                )));
            }
        }
        if let Some(settings) = &self.column_settings {
            if settings.len() > MAX_TABLE_COLUMNS {
                return Err(BuildError::message(format!(
                    "table block column_settings supports at most {} entries",
                    MAX_TABLE_COLUMNS
                )));
            }
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
