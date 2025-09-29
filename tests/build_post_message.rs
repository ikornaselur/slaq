#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::post_message::PostMessage;
use slaq::blocks;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_post_message_minimal() {
    let channel = "C123".to_string();
    let payload = PostMessage::new(channel.clone());
    let req = payload.build_request();

    assert_eq!(req.path, "/chat.postMessage");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    // Body JSON should only contain the required field
    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    // No optional fields by default
    assert!(!body.contains("text"));
}

#[test]
fn build_post_message_with_options() {
    let channel = "C456".to_string();
    let payload = PostMessage::new(channel)
        .text("hello")
        .username("bot")
        .mrkdwn(true)
        .blocks(vec![
            blocks::Divider::new().build(),
            blocks::Markdown::new("notes").build(),
        ]);
    let req = payload.build_request();

    let json = req.to_json().expect("json");
    assert!(json.contains("\"text\":"));
    assert!(json.contains("hello"));
    assert!(json.contains("\"username\":"));
    assert!(json.contains("bot"));
    assert!(json.contains("\"mrkdwn\":true"));
}

#[test]
fn from_method_into_request() {
    let method = PostMessage::new("C789".to_string()).text("hi");
    let req: SlackRequest<PostMessage> = method.into();
    assert_eq!(req.path, "/chat.postMessage");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"text\":\"hi\""));
}
