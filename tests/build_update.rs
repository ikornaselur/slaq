#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::update::Update;
use slaq::blocks;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_update_minimal() {
    let channel = "C123".to_string();
    let ts = "1405894322.002768".to_string();

    let payload = Update::new(channel.clone(), ts.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.update");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"ts\""));
    assert!(body.contains(&format!("\"{ts}\"")));
}

#[test]
fn build_update_with_options() {
    let payload = Update::new("C999".to_string(), "1405894322.002769".to_string())
        .text("updated")
        .link_names(true)
        .blocks(vec![
            blocks::Divider::new().build(),
            blocks::Markdown::new("updated notes").build(),
        ]);
    let req = payload.build_request();
    let json = req.to_json().expect("json");
    assert!(json.contains("\"text\":\"updated\""));
    assert!(json.contains("\"link_names\":true"));
}

#[test]
fn from_method_into_request_update() {
    let method = Update::new("C999".to_string(), "1405894322.002770".to_string()).text("hi");
    let req: SlackRequest<Update> = method.into();
    assert_eq!(req.path, "/chat.update");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"text\":\"hi\""));
}

