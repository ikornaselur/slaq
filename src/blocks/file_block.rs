use serde::Serialize;

#[slaq_macros::block(kind = "file")]
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct File {
    pub external_id: String,
    pub source: FileSource,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileSource {
    Remote,
}

