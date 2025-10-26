use slaq::{
    actions, context, context_actions, datepicker, divider, file, header, image, input, markdown,
    mrkdwn, plain, rich_text, section, table, video,
};

#[test]
fn header_macro_matches_builder() {
    let macro_block = header!("Status").expect("header");
    let builder_block = slaq::blocks::Header::new(plain!("Status")).build().unwrap();
    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn divider_macro_matches_builder() {
    let macro_block = divider!().unwrap();
    let builder_block = slaq::blocks::Divider::new().build().unwrap();
    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn video_macro_matches_builder() {
    let macro_block = video!(
        title = "Highlights",
        video_url = "https://example.com/embed/abc",
        thumbnail_url = "https://example.com/thumb.jpg",
        alt_text = "Product demo",
        description = "Slack sure is nifty!",
        provider_name = "ExampleTube",
    )
    .unwrap();

    let builder_block = slaq::blocks::Video::new(
        plain!("Highlights"),
        "https://example.com/embed/abc",
        "https://example.com/thumb.jpg",
        "Product demo",
    )
    .description(plain!("Slack sure is nifty!"))
    .provider_name("ExampleTube")
    .build()
    .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn section_macro_matches_builder() {
    let macro_block = section!(
        text = mrkdwn!("*Hello*"),
        fields = vec![mrkdwn!("Field1").into(), mrkdwn!("Field2").into()],
    )
    .unwrap();

    let builder_block = slaq::blocks::Section::new()
        .text(mrkdwn!("*Hello*"))
        .fields(vec![mrkdwn!("Field1").into(), mrkdwn!("Field2").into()])
        .build()
        .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn image_macro_matches_builder_with_url() {
    let macro_block = image!(
        alt_text = "Preview",
        image_url = "https://example.com/cat.png",
        title = "cat",
    )
    .unwrap();

    let builder_block = slaq::blocks::Image::new("Preview")
        .image_url("https://example.com/cat.png")
        .title(plain!("cat"))
        .build()
        .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn image_macro_matches_builder_with_slack_file() {
    let macro_block = image!(
        alt_text = "Preview",
        slack_file_id = "F123",
        slack_file_url = "https://files.slack.com/files-pri/T1-F123/image.png",
    )
    .unwrap();

    let builder_block = slaq::blocks::Image::new("Preview")
        .slack_file(slaq::blocks::SlackFileRef {
            url: Some("https://files.slack.com/files-pri/T1-F123/image.png".into()),
            id: Some("F123".into()),
        })
        .build()
        .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn file_macro_matches_builder() {
    let macro_block = file!(external_id = "ABCD1", block_id = "block-1",).unwrap();

    let builder_block =
        slaq::blocks::File::new("ABCD1".to_string(), slaq::blocks::FileSource::Remote)
            .block_id("block-1")
            .build()
            .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn context_macro_matches_builder() {
    let macro_block = context!(
        elements = [
            "*Location*: Dogpatch",
            image(
                "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
                "pin"
            ),
        ],
        block_id = "ctx-1",
    )
    .unwrap();

    let builder_block = slaq::blocks::Context::new(vec![
        slaq::blocks::ContextElement::plain_text("*Location*: Dogpatch"),
        slaq::blocks::ContextElement::image(
            "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
            "pin",
        ),
    ])
    .block_id("ctx-1")
    .build()
    .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn markdown_macro_matches_builder() {
    let m = markdown!("Hello", block_id = "m1").unwrap();
    let b = slaq::blocks::Markdown::new("Hello")
        .block_id("m1")
        .build()
        .unwrap();
    assert_eq!(m.to_value(), b.to_value());
}

#[test]
fn input_macro_matches_builder() {
    let elem = datepicker!("date");
    let m = input!("Date", element = elem, hint = "Optional").unwrap();
    let b = slaq::blocks::Input::new(
        slaq::blocks::PlainText::new("Date"),
        slaq::blocks::BlockElement::from(slaq::blocks::elements::DatePickerElement::new("date")),
    )
    .hint(slaq::blocks::PlainText::new("Optional"))
    .build()
    .unwrap();
    assert_eq!(m.to_value(), b.to_value());
}

#[test]
fn table_macro_matches_builder() {
    let rows = vec![
        vec![
            slaq::blocks::TableCell::raw("A"),
            slaq::blocks::TableCell::raw("B"),
        ],
        vec![
            slaq::blocks::TableCell::raw("C"),
            slaq::blocks::TableCell::raw("D"),
        ],
    ];
    let m = table!(rows = rows.clone(), block_id = "t1").unwrap();
    let b = slaq::blocks::Table::new(rows)
        .block_id("t1")
        .build()
        .unwrap();
    assert_eq!(m.to_value(), b.to_value());
}

#[test]
fn rich_text_macro_matches_builder() {
    let elem = slaq::blocks::rich_text::RichTextElement::section(vec![
        slaq::blocks::rich_text::RichTextNode::text("Hi"),
    ]);
    let m = rich_text!([elem.clone()], block_id = "r1").unwrap();
    let b = slaq::blocks::RichText::new(vec![elem])
        .block_id("r1")
        .build()
        .unwrap();
    assert_eq!(m.to_value(), b.to_value());
}

#[test]
fn actions_macro_matches_builder() {
    let select = slaq::blocks::elements::StaticSelectElement::new("sel").options(vec![
        slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("A"), "a"),
    ]);
    let button = slaq::blocks::ButtonElement::new(slaq::blocks::PlainText::new("Go"), "go");

    let macro_block = actions!([select, button], block_id = "act-1").unwrap();

    let builder_block = slaq::blocks::Actions::new(vec![
        slaq::blocks::BlockElement::from(
            slaq::blocks::elements::StaticSelectElement::new("sel").options(vec![
                slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("A"), "a"),
            ]),
        ),
        slaq::blocks::BlockElement::from(slaq::blocks::ButtonElement::new(
            slaq::blocks::PlainText::new("Go"),
            "go",
        )),
    ])
    .block_id("act-1")
    .build()
    .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn context_actions_macro_matches_builder() {
    use slaq::blocks::elements::{ContextActionElement, FeedbackButton};
    let pos = FeedbackButton::new("👍", "positive");
    let neg = FeedbackButton::new("👎", "negative");
    let macro_block = context_actions!([ContextActionElement::feedback(
        "fb_1",
        pos.clone(),
        neg.clone()
    )])
    .unwrap();
    let builder_block =
        slaq::blocks::ContextActions::new(vec![ContextActionElement::feedback("fb_1", pos, neg)])
            .build()
            .unwrap();
    assert_eq!(macro_block.to_value(), builder_block.to_value());
}
