#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::delete::Delete;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_delete_minimal() {
    let channel = "C123".to_string();
    let ts = "1727612345.000200".to_string();

    let payload = Delete::new(channel.clone(), ts.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.delete");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    // Body JSON should only contain the required fields by default
    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"ts\""));
    assert!(body.contains(&format!("\"{ts}\"")));
    assert!(!body.contains("as_user"));
}

#[test]
fn from_method_into_request_delete() {
    let method = Delete::new("C999".to_string(), "1727612345.000300".to_string()).as_user(true);
    let req: SlackRequest<Delete> = method.into();
    assert_eq!(req.path, "/chat.delete");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"as_user\":true"));
}
