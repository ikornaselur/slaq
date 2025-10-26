use slaq::blocks::BuildError;
use slaq::{blocks, divider, fields, header, mrkdwn, section, video};

fn main() -> Result<(), BuildError> {
    let blocks = blocks![
        header!("Status Report"),
        divider!(),
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
