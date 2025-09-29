#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::scheduled_messages_list::ScheduledMessagesList;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_scheduled_messages_list_minimal() {
    let payload = ScheduledMessagesList::new();
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.scheduledMessages.list");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    let body = req.to_json().expect("json");
    // No optional fields by default
    assert_eq!(body, "{}");
}

#[test]
fn build_scheduled_messages_list_with_options() {
    let payload = ScheduledMessagesList::new()
        .channel("C123")
        .cursor("abc")
        .limit(50u32)
        .oldest("1562137200")
        .latest("1562138200");
    let req = payload.build_request();
    let json = req.to_json().expect("json");
    assert!(json.contains("\"channel\":\"C123\""));
    assert!(json.contains("\"cursor\":\"abc\""));
    assert!(json.contains("\"limit\":50"));
    assert!(json.contains("\"oldest\":\"1562137200\""));
    assert!(json.contains("\"latest\":\"1562138200\""));
}

#[test]
fn from_method_into_request_scheduled_messages_list() {
    let method = ScheduledMessagesList::new().limit(10u32);
    let req: SlackRequest<ScheduledMessagesList> = method.into();
    assert_eq!(req.path, "/chat.scheduledMessages.list");
}
