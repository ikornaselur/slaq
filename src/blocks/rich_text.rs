use serde::Serialize;

/// Top-level elements inside a rich text block.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextElement {
    RichTextSection {
        elements: Vec<RichTextNode>,
    },
    RichTextList {
        style: ListStyle,
        elements: Vec<RichTextElement>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        indent: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        offset: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        border: Option<u32>,
    },
    RichTextPreformatted {
        elements: Vec<RichTextNode>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        border: Option<u32>,
    },
    RichTextQuote {
        elements: Vec<RichTextNode>,
    },
}

impl RichTextElement {
    #[must_use]
    pub fn section(nodes: impl Into<Vec<RichTextNode>>) -> Self {
        RichTextElement::RichTextSection {
            elements: nodes.into(),
        }
    }

    #[must_use]
    pub fn list(style: ListStyle, items: impl Into<Vec<RichTextElement>>) -> Self {
        RichTextElement::RichTextList {
            style,
            elements: items.into(),
            indent: None,
            offset: None,
            border: None,
        }
    }

    #[must_use]
    pub fn preformatted(nodes: impl Into<Vec<RichTextNode>>) -> Self {
        RichTextElement::RichTextPreformatted {
            elements: nodes.into(),
            border: None,
        }
    }

    #[must_use]
    pub fn quote(nodes: impl Into<Vec<RichTextNode>>) -> Self {
        RichTextElement::RichTextQuote {
            elements: nodes.into(),
        }
    }

    #[must_use]
    pub fn indent(mut self, value: u32) -> Self {
        if let RichTextElement::RichTextList { indent, .. } = &mut self {
            *indent = Some(value);
        }
        self
    }

    #[must_use]
    pub fn offset(mut self, value: u32) -> Self {
        if let RichTextElement::RichTextList { offset, .. } = &mut self {
            *offset = Some(value);
        }
        self
    }

    #[must_use]
    pub fn border(mut self, value: u32) -> Self {
        match &mut self {
            RichTextElement::RichTextList { border, .. }
            | RichTextElement::RichTextPreformatted { border, .. } => *border = Some(value),
            _ => {}
        }
        self
    }
}

/// Nodes within rich text sections/lists/preformatted content.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextNode {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        style: Option<TextStyle>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        url: Option<String>,
    },
    Link {
        url: String,
        text: String,
    },
    Emoji {
        name: String,
    },
    User {
        user_id: String,
    },
    Broadcast {
        range: BroadcastRange,
    },
    Channel {
        channel_id: String,
    },
}

impl RichTextNode {
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        RichTextNode::Text {
            text: text.into(),
            style: None,
            url: None,
        }
    }

    #[must_use]
    pub fn styled_text(text: impl Into<String>, style: TextStyle) -> Self {
        RichTextNode::Text {
            text: text.into(),
            style: Some(style),
            url: None,
        }
    }

    #[must_use]
    pub fn link(text: impl Into<String>, url: impl Into<String>) -> Self {
        RichTextNode::Link {
            text: text.into(),
            url: url.into(),
        }
    }

    #[must_use]
    pub fn emoji(name: impl Into<String>) -> Self {
        RichTextNode::Emoji { name: name.into() }
    }

    #[must_use]
    pub fn user(user_id: impl Into<String>) -> Self {
        RichTextNode::User {
            user_id: user_id.into(),
        }
    }

    #[must_use]
    pub fn broadcast(range: BroadcastRange) -> Self {
        RichTextNode::Broadcast { range }
    }

    #[must_use]
    pub fn channel(channel_id: impl Into<String>) -> Self {
        RichTextNode::Channel {
            channel_id: channel_id.into(),
        }
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        if let RichTextNode::Text { url: slot, .. } = &mut self {
            *slot = Some(url.into());
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListStyle {
    Bullet,
    Ordered,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub strike: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub code: Option<bool>,
}

impl TextStyle {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }

    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }

    #[must_use]
    pub fn strike(mut self) -> Self {
        self.strike = Some(true);
        self
    }

    #[must_use]
    pub fn code(mut self) -> Self {
        self.code = Some(true);
        self
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastRange {
    Here,
    Channel,
    Everyone,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_serializes_text_nodes() {
        let element = RichTextElement::section(vec![
            RichTextNode::text("Hello"),
            RichTextNode::styled_text("world", TextStyle::new().bold()),
        ]);

        let json = serde_json::to_string(&element).unwrap();
        assert!(json.contains("\"type\":\"rich_text_section\""));
        assert!(json.contains("Hello"));
        assert!(json.contains("world"));
    }
}
