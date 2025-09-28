#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::post_ephemeral::PostEphemeral;
use slaq::client::{Encoding, HttpMethod};

#[test]
fn build_post_ephemeral_minimal() {
    let channel = "C123".to_string();
    let user = "U123".to_string();

    let payload = PostEphemeral::new(channel, user);
    let req = payload.build_request();
    assert_eq!(req.path, "/chat.postEphemeral");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));
}
