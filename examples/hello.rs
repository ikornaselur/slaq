use std::env;

use slaq::api::chat::post_message::PostMessage;
use slaq::blocks;
use slaq::client::Execute;
use slaq::{Client, DEFAULT_BASE_URL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("SLACK_BOT_TOKEN")?;
    let channel = env::var("SLACK_CHANNEL")?;

    let client = Client::new(DEFAULT_BASE_URL, token);

    let payload = PostMessage::new(channel).text("Hello, world").blocks(vec![
        blocks::Markdown::new("Hello from slaq examples").build(),
        blocks::Divider::new().build(),
    ]);

    let resp = client.execute(payload)?;
    println!("sent: {resp:?}");
    Ok(())
}
