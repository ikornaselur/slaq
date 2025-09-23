use serde::Serialize;
use std::error::Error;

pub struct Client {
    client: reqwest::blocking::Client,
    base_url: String,
    token: String,
}

impl Client {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.into(),
            token: token.into(),
        }
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn get_path(&self, payload: &T) -> String {
        // TODO: How to?
    }

    pub fn post<T: Serialize>(&self, path: &str, payload: &T) -> Result<String, Box<dyn Error>> {
        let url = self.get_url(path);

        let res = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .json(payload)
            .send()?
            .text()?;

        Ok(res)
    }
}
