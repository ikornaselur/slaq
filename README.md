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

All Block Kit blocks are supported with typed builders (divider, markdown, header, image, file, context, context_actions, actions, input, video, rich_text, table). Elements are supported as typed builders, too (buttons, selects, inputs, choices, etc.).

Builder example:

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

Macro helpers (blocks):

```rust
use slaq::blocks::{self, divider, header, markdown, section, video, image, file, actions, context, context_actions};
use slaq::blocks::text::{mrkdwn, fields};

let blocks = blocks::blocks_vec![
    header!("Greetings"),
    markdown!("Hello"),
    image!(alt_text = "Kittens", image_url = "https://placekitten.com/200/300"),
    divider!(),
    section!(text = mrkdwn!("A message *with some rich text*"), fields = fields![mrkdwn!("*Env:* prod")]),
    video!(
        title = "Demo",
        video_url = "https://example.com/embed/abc",
        thumbnail_url = "https://example.com/thumb.jpg",
        alt_text = "Product demo",
    ),
]?;
```

Macro helpers (elements and actions):

```rust
use slaq::blocks::elements::{option, options, button, select, external_select, overflow, datepicker, timepicker};
use slaq::blocks::text::mrkdwn;

let act = blocks::actions!([
    select!("sel_1", placeholder = "Pick", options = options![option!("A","a"), option!("B","b")]),
    button!("Confirm", "btn_confirm"),
    overflow!("more", options = options![option!("Edit","edit"), option!("Delete","delete")]),
])?;
```

Notes on text defaults:

- `section!`: string text is treated as mrkdwn (Slack’s default for section).
- `context!`: string items are treated as `plain_text`; use `mrkdwn!(...)` to opt into markdown.
- `header!`, `markdown!`: strings map to `plain_text` and raw string respectively.







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

Macros quick start
------------------

- Blocks macros example: `examples/blocks_macros.rs`
  - Shows header!/divider!/section!/video!/image! with text helpers (plain!, mrkdwn!) and fields!
- Elements macros example: `examples/elements_macros.rs`
  - Shows select!/external_select!/button!/overflow!/datepicker!/timepicker! used as section accessories
  - You can also compose an actions block with actions!([...]) using element macros directly

Macro Reference
---------------

- Blocks
  - `header!(text[, block_id?=...])`
  - `divider!([block_id?=...])`
  - `markdown!(text[, block_id?=...])`
  - `section!(text?=..., fields?=..., accessory?=..., expand?=..., block_id?=...)`
  - `image!(alt_text=..., image_url=... | slack_file_id=..., slack_file_url=... [, title?=..., block_id?=...])`
  - `file!(external_id=... [, block_id?=...])`
  - `video!(title=..., video_url=..., thumbnail_url=..., alt_text=... [, description?=..., title_url?=..., provider_icon_url?=..., provider_name?=..., author_name?=..., block_id?=...])`
  - `rich_text!(elements = vec![...][, block_id?=...])`
  - `table!(rows = vec![...][, column_settings = vec![...]][, block_id?=...])`
  - `actions!([element, ...][, block_id?=...])`
  - `context_actions!([element, ...][, block_id?=...])`
  - `blocks_vec![...]`

- Text Helpers
  - `plain!(text)`
  - `mrkdwn!(text[, verbatim?=bool])`
  - `fields![...]`

- Elements
  - Buttons
    - `button!(text, action_id[, style?=..., url?=..., value?=..., confirm?=...])`
  - Selects
    - `select!(action_id[, placeholder?=..., options?=options![...], option_groups?=..., initial_option?=..., confirm?=...])`
    - `multiselect!(...)`
    - `external_select!(...)`
    - `multi_external_select!(...)`
    - `users_select!(...)`
    - `multi_users_select!(...)`
    - `conversations_select!(...[, filter?=...])`
    - `multi_conversations_select!(...)`
    - `channels_select!(...)`
    - `multi_channels_select!(...)`
  - Inputs
    - `datepicker!(...)`
    - `timepicker!(...)`
    - `datetimepicker!(...)`
    - `plain_text_input!(...)`
    - `email_input!(...)`
    - `url_input!(...)`
    - `number_input!(...)`
  - Choices
    - `checkboxes!(action_id, options = options![...][, initial_options?=..., confirm?=...])`
    - `radio_buttons!(action_id, options = options![...][, initial_option?=..., confirm?=...])`
    - `overflow!(action_id, options = options![...][, confirm?=...])`
  - Composition
    - `option!(text, value)`
    - `options![...]`
    - `option_group!(label, options![...])`
    - `confirm!(title, text, confirm, deny)`

Macro Notes & Edge Cases
------------------------

- Text defaults: `section!(text = "...")` treats string as mrkdwn (Slack’s default). `context!(["..."])` treats strings as plain text. Use `mrkdwn!(...)` when you need markdown explicitly.
- Required keys: some macros validate required keys. For example, `video!` requires `title`, `video_url`, `thumbnail_url`, and `alt_text`. Missing any will produce a clear error at build time.
- Types vs. macros: macros accept strings for convenience. If you need exact schema types (e.g., `plain_text` objects), prefer the builder API or the `plain!`/`mrkdwn!` helpers.
- Collectors: prefer `fields![...]` and `blocks_vec![...]` to avoid `.into()` noise and to collect results, respectively.
- Select menus: when using static selects (single or multi), provide either `options = options![...]` or `option_groups = vec![...]` (not both). Builders enforce this when embedded in a block.

Notes
-----

- Encoding is currently JSON; `req.content_type()` returns `application/json`
  and `req.to_json()` produces the body.
- The built-in client is blocking. If you need async or a different transport,
  use build-only mode and plug in your HTTP stack.
- Responses: Slack responses include an `ok` field. The built-in client handles
  decoding and error mapping. In build-only mode, you receive the raw response
  and should handle Slack errors yourself.
