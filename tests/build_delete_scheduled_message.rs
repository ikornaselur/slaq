#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::delete_scheduled_message::DeleteScheduledMessage;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_delete_scheduled_message_minimal() {
    let channel = "C123".to_string();
    let scheduled_message_id = "Q1234ABCD".to_string();

    let payload = DeleteScheduledMessage::new(channel.clone(), scheduled_message_id.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.deleteScheduledMessage");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    // Body JSON should only contain required fields by default
    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"scheduled_message_id\""));
    assert!(body.contains(&format!("\"{scheduled_message_id}\"")));
    assert!(!body.contains("as_user"));
}

#[test]
fn from_method_into_request_delete_scheduled_message() {
    let method =
        DeleteScheduledMessage::new("C999".to_string(), "Q0000AAAA".to_string()).as_user(true);
    let req: SlackRequest<DeleteScheduledMessage> = method.into();
    assert_eq!(req.path, "/chat.deleteScheduledMessage");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"as_user\":true"));
}
