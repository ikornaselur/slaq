use slaq::blocks::BuildError;
use slaq::{blocks, context, divider, fields, file, header, image, mrkdwn, section, video};

fn main() -> Result<(), BuildError> {
    let blocks = blocks![
        header!("Status Report"),
        divider!(),
        file!(
            external_id = "file-ext-123",
        ),
        image!(
            alt_text = "Kittens",
            image_url = "https://placekitten.com/200/300",
            title = "Cute!",
        ),
        context!([
            "Env: prod",
            image("https://example.com/icon.png", "icon"),
            plain("v1.2.3"),
            mrkdwn("*Status:* green"),
        ]),
        video!(
            title = "Highlights",
            video_url = "https://example.com/embed/abc",
            thumbnail_url = "https://example.com/thumb.jpg",
            alt_text = "Product demo",
            description = "Slack sure is nifty!",
            provider_name = "ExampleTube",
            title_url = "https://example.com/watch",
        ),
        section!(
            text = mrkdwn!("A message *with some rich text*"),
            fields = fields![
                mrkdwn!("*Environment:* production"),
                mrkdwn!("*Status:* green"),
            ],
        ),
    ]?;

    for block in blocks.into_iter() {
        println!(
            "{}",
            serde_json::to_string_pretty(&block.to_value()).unwrap()
        );
    }

    Ok(())
}
