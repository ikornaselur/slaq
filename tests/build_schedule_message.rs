#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::schedule_message::ScheduleMessage;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_schedule_message_minimal() {
    let channel = "C123".to_string();
    let post_at: i64 = 299876400;

    let payload = ScheduleMessage::new(channel.clone(), post_at.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.scheduleMessage");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"post_at\""));
    assert!(body.contains(&format!("{post_at}")));
}

#[test]
fn from_method_into_request_schedule_message() {
    let method = ScheduleMessage::new("C999".to_string(), 299876401i64).text("hi later");
    let req: SlackRequest<ScheduleMessage> = method.into();
    assert_eq!(req.path, "/chat.scheduleMessage");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"text\":\"hi later\""));
}
