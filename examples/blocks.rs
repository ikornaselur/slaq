use slaq::blocks::{
    Actions, BlockElement, ButtonElement, ButtonStyle, ColumnAlignment, ColumnSetting, Context,
    ContextElement, Divider, Header, Image, PlainText, RichText, RichTextElement, RichTextNode,
    Table, TableCell, TextStyle, Video,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let header = Header::new(PlainText::new("Status Report")).build()?;
    let divider = Divider::new().build()?;
    let image = Image::new("A cute kitten")
        .image_url("https://placekitten.com/200/300")
        .build()?;
    let context = Context::new(vec![
        ContextElement::mrkdwn("*Environment*: production"),
        ContextElement::mrkdwn("*Status*: green"),
    ])
    .build()?;
    let video = Video::new(
        PlainText::new("Highlights"),
        "https://example.com/embed/abc",
        "https://example.com/thumb.jpg",
        "Product demo",
    )
    .build()?;
    let rich_text = RichText::new(vec![RichTextElement::section(vec![
        RichTextNode::text("* Hello from rich text *"),
        RichTextNode::styled_text("Primary CTA", TextStyle::new().bold().italic()),
    ])])
    .build()?;
    let table = Table::new(vec![
        vec![TableCell::raw("Service"), TableCell::raw("Status")],
        vec![TableCell::raw("API"), TableCell::raw("Operational")],
    ])
    .column_settings(vec![
        ColumnSetting::new().align(ColumnAlignment::Left),
        ColumnSetting::new().align(ColumnAlignment::Left),
    ])
    .build()?;
    let actions = Actions::new(vec![BlockElement::from(
        ButtonElement::new(PlainText::new("Acknowledge"), "ack").style(ButtonStyle::Primary),
    )])
    .build()?;

    let blocks = vec![
        header, divider, image, video, context, rich_text, table, actions,
    ];
    for block in blocks {
        let json = serde_json::to_string_pretty(&block.to_value())?;
        println!("{}", json);
    }

    Ok(())
}
