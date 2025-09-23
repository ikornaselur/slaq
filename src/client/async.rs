pub struct Client {
    base_url: String,
    token: String,
}

impl Client {
    pub fn new(_base_url: impl Into<String>, _token: impl Into<String>) -> Self {
        unimplemented!("async client not implemented yet")
    }
}
