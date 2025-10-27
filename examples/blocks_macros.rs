use slaq::blocks;
use slaq::blocks::BuildError;
use slaq::blocks::{context, divider, file, header, image, section, video};

fn main() -> Result<(), BuildError> {
    let blocks = blocks::blocks_vec![
        header!("Status Report"),
        divider!(),
        file!(external_id = "file-ext-123",),
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
            text = blocks::text::mrkdwn!("A message *with some rich text*"),
            fields = blocks::text::fields![
                blocks::text::mrkdwn!("*Environment:* production"),
                blocks::text::mrkdwn!("*Status:* green"),
            ],
        ),
    ]?;

    for block in blocks {
        println!(
            "{}",
            serde_json::to_string_pretty(&block.to_value()).unwrap()
        );
    }

    Ok(())
}
