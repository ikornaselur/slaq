use crate::client::method::{Encoding, Execute, HttpMethod, SlackMethod};

pub struct Client {
    client: reqwest::blocking::Client,
    base_url: String,
    token: String,
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl Client {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.into(),
            token: token.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn _execute_internal<M: SlackMethod>(&self, method: M) -> Result<M::Response> {
        let body = method.into_body();
        let url = self.url(M::PATH);
        let req = match M::method() {
            HttpMethod::Post => self.client.post(url),
            HttpMethod::Get => self.client.get(url),
        }
        .bearer_auth(&self.token);

        let resp = match M::encoding() {
            Encoding::Json => req.json(&body).send()?,
        };

        let parsed = resp.json::<M::Response>()?;
        Ok(parsed)
    }
}

impl Execute for Client {
    type Error = Error;

    fn execute<M: SlackMethod>(&self, method: M) -> Result<M::Response> {
        self._execute_internal(method)
    }
}
