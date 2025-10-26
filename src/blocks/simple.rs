use serde::Serialize;

use crate::blocks::text::PlainText;

#[slaq_macros::block(kind = "divider")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Divider {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "markdown")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Markdown {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[slaq_macros::block(kind = "header")]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub text: PlainText,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

