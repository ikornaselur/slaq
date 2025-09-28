#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::post_message::PostMessage;
use slaq::api::chat::ChatExt;
use slaq::client::{Encoding, HttpMethod, SlackMethod, SlackRequest};

struct NoopClient;

impl slaq::client::Execute for NoopClient {
    type Error = ();
    fn execute<M: SlackMethod>(&self, _method: M) -> Result<M::Response, Self::Error> {
        unreachable!("send should not be called in build tests")
    }
}

#[test]
fn build_post_message_minimal() {
    let client = NoopClient;
    let channel = "C123".to_string();

    let chat = client.chat();
    let call = chat.post_message(channel.clone());
    let req = call.build();

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
    let client = NoopClient;
    let channel = "C456".to_string();

    let chat = client.chat();
    let req = chat
        .post_message(channel)
        .text("hello")
        .username("bot")
        .mrkdwn(true)
        .build();

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
