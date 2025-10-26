use serde_json::{Value, json};

use slaq::blocks::{
    Actions, Block, BlockElement, ButtonElement, ColumnAlignment, ColumnSetting, PlainText,
    RichTextElement, RichTextNode, Section, StaticSelectElement, Table, TableCell, Video,
};

fn assert_json_eq(block: &Block, expected: &Value) {
    let actual = block.to_value();
    assert_eq!(&actual, expected);
}

#[test]
fn section_doc_example() {
    let block = Section::new()
        .text("A message *with some bold text* and _some italicized text_.")
        .build()
        .expect("section build");

    let expected = json!({
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": "A message *with some bold text* and _some italicized text_."
        }
    });

    assert_json_eq(&block, &expected);
}

#[test]
fn actions_doc_example() {
    let select = StaticSelectElement::new("select_2")
        .placeholder(PlainText::new("Which witch is the witchiest witch?"))
        .options(vec![
            slaq::blocks::SelectOption::new(PlainText::new("Matilda"), "matilda"),
            slaq::blocks::SelectOption::new(PlainText::new("Glinda"), "glinda"),
            slaq::blocks::SelectOption::new(
                PlainText::new("Granny Weatherwax"),
                "grannyWeatherwax",
            ),
            slaq::blocks::SelectOption::new(PlainText::new("Hermione"), "hermione"),
        ]);

    let button = ButtonElement::new(PlainText::new("Cancel"), "button_1").value("cancel");

    let block = Actions::new(vec![BlockElement::from(select), BlockElement::from(button)])
        .block_id("actions1")
        .build()
        .expect("actions build");

    let expected = json!({
        "type": "actions",
        "block_id": "actions1",
        "elements": [
            {
                "type": "static_select",
                "placeholder": {"type": "plain_text", "text": "Which witch is the witchiest witch?"},
                "action_id": "select_2",
                "options": [
                    {"text": {"type": "plain_text", "text": "Matilda"}, "value": "matilda"},
                    {"text": {"type": "plain_text", "text": "Glinda"}, "value": "glinda"},
                    {"text": {"type": "plain_text", "text": "Granny Weatherwax"}, "value": "grannyWeatherwax"},
                    {"text": {"type": "plain_text", "text": "Hermione"}, "value": "hermione"}
                ]
            },
            {
                "type": "button",
                "text": {"type": "plain_text", "text": "Cancel"},
                "value": "cancel",
                "action_id": "button_1"
            }
        ]
    });

    assert_json_eq(&block, &expected);
}

#[test]
fn video_doc_example() {
    let block = Video::new(
        PlainText::new("Use the Events API to create a dynamic App Home"),
        "https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1",
        "https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg",
        "Use the Events API to create a dynamic App Home",
    )
    .title_url("https://www.youtube.com/watch?v=8876OZV_Yy0")
    .description(PlainText::new("Slack sure is nifty!"))
    .build()
    .expect("video build");

    let expected = json!({
        "type": "video",
        "title": {"type": "plain_text", "text": "Use the Events API to create a dynamic App Home"},
        "title_url": "https://www.youtube.com/watch?v=8876OZV_Yy0",
        "description": {"type": "plain_text", "text": "Slack sure is nifty!"},
        "video_url": "https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1",
        "alt_text": "Use the Events API to create a dynamic App Home",
        "thumbnail_url": "https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg"
    });

    assert_json_eq(&block, &expected);
}

#[test]
fn table_doc_example() {
    let rich_link =
        RichTextElement::section(vec![RichTextNode::link("Data 1B", "https://slack.com")]);

    let rows = vec![
        vec![TableCell::raw("Header A"), TableCell::raw("Header B")],
        vec![
            TableCell::raw("Data 1A"),
            TableCell::rich(vec![rich_link.clone()]),
        ],
        vec![TableCell::raw("Data 2A"), TableCell::rich(vec![rich_link])],
    ];

    let block = Table::new(rows)
        .column_settings(vec![
            ColumnSetting::new().wrap(true),
            ColumnSetting::new().align(ColumnAlignment::Right),
        ])
        .build()
        .expect("table build");

    let expected = json!({
        "type": "table",
        "column_settings": [
            {"is_wrapped": true},
            {"align": "right"}
        ],
        "rows": [
            [
                {"type": "raw_text", "text": "Header A"},
                {"type": "raw_text", "text": "Header B"}
            ],
            [
                {"type": "raw_text", "text": "Data 1A"},
                {"type": "rich_text", "elements": [
                    {"type": "rich_text_section", "elements": [
                        {"type": "link", "text": "Data 1B", "url": "https://slack.com"}
                    ]}
                ]}
            ],
            [
                {"type": "raw_text", "text": "Data 2A"},
                {"type": "rich_text", "elements": [
                    {"type": "rich_text_section", "elements": [
                        {"type": "link", "text": "Data 1B", "url": "https://slack.com"}
                    ]}
                ]}
            ]
        ]
    });

    assert_json_eq(&block, &expected);
}
