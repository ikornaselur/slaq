#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::me_message::MeMessage;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_me_message_minimal() {
    let channel = "C123".to_string();
    let text = "waves".to_string();

    let payload = MeMessage::new(channel.clone(), text.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.meMessage");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"text\""));
    assert!(body.contains(&format!("\"{text}\"")));
}

#[test]
fn from_method_into_request_me_message() {
    let method = MeMessage::new("C999".to_string(), "does a thing".to_string());
    let req: SlackRequest<MeMessage> = method.into();
    assert_eq!(req.path, "/chat.meMessage");
}

