#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::unfurl::Unfurl;
use slaq::client::{Encoding, HttpMethod, SlackRequest};

#[test]
fn build_unfurl_minimal() {
    let channel = "C123".to_string();
    let ts = "1727612345.000200".to_string();
    let unfurls = "{\"https://example.com\":{\"blocks\":[...]}}".to_string();

    let payload = Unfurl::new(channel.clone(), ts.clone(), unfurls.clone());
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.unfurl");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));

    let body = req.to_json().expect("json");
    assert!(body.contains("\"channel\""));
    assert!(body.contains(&format!("\"{channel}\"")));
    assert!(body.contains("\"ts\""));
    assert!(body.contains(&format!("\"{ts}\"")));
    assert!(body.contains("\"unfurls\""));
}

#[test]
fn from_method_into_request_unfurl() {
    let method = Unfurl::new("C999".to_string(), "1727612345.000300".to_string(), "{}".to_string())
        .user_auth_required(true);
    let req: SlackRequest<Unfurl> = method.into();
    assert_eq!(req.path, "/chat.unfurl");
    let json = req.to_json().expect("json");
    assert!(json.contains("\"user_auth_required\":true"));
}

