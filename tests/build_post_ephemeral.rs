#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use slaq::api::chat::ChatExt;
use slaq::client::{Encoding, HttpMethod, SlackMethod};

struct NoopClient;

impl slaq::client::Execute for NoopClient {
    type Error = ();
    fn execute<M: SlackMethod>(&self, _method: M) -> Result<M::Response, Self::Error> {
        unreachable!("send should not be called in build tests")
    }
}

#[test]
fn build_post_ephemeral_minimal() {
    let client = NoopClient;
    let channel = "C123".to_string();
    let user = "U123".to_string();

    let chat = client.chat();
    let req = chat.post_ephemeral(channel, user).build();
    assert_eq!(req.path, "/chat.postEphemeral");
    assert!(matches!(req.method, HttpMethod::Post));
    assert!(matches!(req.encoding, Encoding::Json));
}
