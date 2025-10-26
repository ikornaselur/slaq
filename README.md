Slaq: A Slack API Client
====

> [!WARNING]
> This is an experimental crate for now, with plans to expand into a
> library that fully supports the Slack API.

Typed Slack Web API payload builders with an optional reqwest transport. Build
request payloads in Rust, then send them with the provided blocking client or
with any HTTP client you prefer.

Features
--------

- Core builders (default): typed Slack Web API payload builders without any HTTP client.
- `transport-reqwest`: enables the built-in blocking client using reqwest.

Install
-------

Core builders only (default, bring your own HTTP client):

```
[dependencies]
slaq = "0.0.3"
```

With the built-in reqwest transport:

```
[dependencies]
slaq = { version = "0.0.3", features = ["transport-reqwest"] }
```

Quick Start
-----------

With the built-in client (enable the `transport-reqwest` feature):

```rust
// Cargo.toml: slaq = { version = "0.0.3", features = ["transport-reqwest"] }
use slaq::{Client, DEFAULT_BASE_URL};
use slaq::api::chat::post_message::PostMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("SLACK_BOT_TOKEN")?;
    let channel = "C01234567".to_string();

    let client = Client::new(DEFAULT_BASE_URL, token);

    // Build payload then execute it
    let payload = PostMessage::new(channel).text("hello from slaq");
    let _resp = client.execute(payload)?;

    Ok(())
}
```

Build, then send with the same client (explicit request):

```rust
use slaq::{Client, DEFAULT_BASE_URL};
use slaq::api::chat::post_message::PostMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("SLACK_BOT_TOKEN")?;
    let channel = "C01234567".to_string();

    let client = Client::new(DEFAULT_BASE_URL, token);

    let req = PostMessage::new(channel).text("hello (built)").build_request();

    // Send previously built request
    let _resp = client.send(&req)?;
    Ok(())
}
```

Supported Methods
-----------------

The following Slack Web API chat.* methods are currently available as typed payload builders:

- chat.postMessage → `api::chat::post_message::PostMessage`
- chat.postEphemeral → `api::chat::post_ephemeral::PostEphemeral`
- chat.delete → `api::chat::delete::Delete`
- chat.deleteScheduledMessage → `api::chat::delete_scheduled_message::DeleteScheduledMessage`
- chat.meMessage → `api::chat::me_message::MeMessage`
- chat.scheduleMessage → `api::chat::schedule_message::ScheduleMessage`
- chat.scheduledMessages.list → `api::chat::scheduled_messages_list::ScheduledMessagesList`
- chat.unfurl → `api::chat::unfurl::Unfurl`
- chat.update → `api::chat::update::Update`

Blocks
------

This crate includes a growing BlockKit-like builder with several blocks today: divider, markdown, header, image, file, context, context actions, actions, video, rich text, and table.

```rust
use slaq::blocks::{self, BlockElement, ButtonElement, ButtonStyle, PlainText};
use slaq::api::chat::post_message::PostMessage;

let blocks = vec![
    blocks::Header::new(PlainText::new("Greetings")).build()?,
    blocks::Markdown::new("Hello").build()?,
    blocks::Image::new("Kittens!")
        .image_url("https://placekitten.com/200/300")
        .build()?,
    blocks::Divider::new().build()?,
];

let payload = PostMessage::new(channel)
    .text("Hello with blocks")
    .blocks(blocks);
```







Examples
--------

There is a basic example that sends a message:

```
cargo run --example hello
```

It expects the following environment variables:

- `SLACK_BOT_TOKEN`: your app’s bot token
- `SLACK_CHANNEL`: the channel ID to post to (e.g. `C01234567`)

Build-only usage (no reqwest; you send it):

```rust
// Cargo.toml: slaq = "0.0.3"

use slaq::api::chat::post_message::PostMessage;
use slaq::client::SlackRequest; // request wrapper

fn build_request(
  channel: String
) -> serde_json::Result<(SlackRequest<PostMessage>, String)> {
    // Construct the typed method struct
    let method = PostMessage::new(channel).text("hello from build-only");

    // Convert it into a transport-agnostic request
    let req: SlackRequest<PostMessage> = method.into();

    // Serialize for sending with your HTTP client
    let body_json = req.to_json()?;
    Ok((req, body_json))
}

fn send_with_any_http(
    (req, body): (SlackRequest<PostMessage>, String),
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}{}", base_url, req.path);

    // Pseudocode: use any HTTP client to POST
    // - Method: req.method (POST/GET)
    // - URL: url
    // - Header: Authorization: Bearer {token}
    // - Header: Content-Type: req.content_type()
    // - Body: body (JSON string)
    // Example with your chosen client goes here.

    Ok(())
}
```

Blocks-only webhook payload (no Slack token):

```rust
// Cargo.toml:
// slaq = "0.0.3"
// serde_json = "1.0"
// reqwest = { version = "0.12", features = ["blocking", "json", "rustls-tls"] }

use slaq::blocks;
use serde_json::json;

fn send_webhook() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = std::env::var("SLACK_WEBHOOK_URL")?;

    let blocks = vec![
        blocks::Markdown::new("*Release finished*").build()?,
        blocks::Divider::new().build()?,
        blocks::Markdown::new("All services are healthy.").build()?,
    ];

    let payload = json!({
        "text": "Release finished",
        "blocks": blocks,
    });

    reqwest::blocking::Client::new()
        .post(&webhook_url)
        .json(&payload)
        .send()?
        .error_for_status()?;

    Ok(())
}
```

This example only uses the block builders to compose the JSON payload that Slack incoming
webhooks expect. The `text` field provides a plain-text fallback for clients that
do not render blocks.

Notes
-----

- Encoding is currently JSON; `req.content_type()` returns `application/json`
  and `req.to_json()` produces the body.
- The built-in client is blocking. If you need async or a different transport,
  use build-only mode and plug in your HTTP stack.
- Responses: Slack responses include an `ok` field. The built-in client handles
  decoding and error mapping. In build-only mode, you receive the raw response
  and should handle Slack errors yourself.
