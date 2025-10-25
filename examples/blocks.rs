use slaq::blocks::{
    Actions, BlockElement, ButtonElement, ButtonStyle, Context, ContextElement, Divider, Header,
    Image, PlainText,
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
    let actions = Actions::new(vec![BlockElement::from(
        ButtonElement::new(PlainText::new("Acknowledge"), "ack").style(ButtonStyle::Primary),
    )])
    .build()?;

    let blocks = vec![header, divider, image, context, actions];
    for block in blocks {
        let json = serde_json::to_string_pretty(&block.to_value())?;
        println!("{}", json);
    }

    Ok(())
}
