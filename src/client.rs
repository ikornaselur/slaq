use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Copy, Clone, Debug)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Copy, Clone, Debug)]
pub enum Encoding {
    Json,
}

pub trait SlackMethod {
    const PATH: &'static str;
    type Body: Serialize;
    type Response: DeserializeOwned;
    fn into_body(self) -> Self::Body;
    #[must_use]
    fn method() -> HttpMethod {
        HttpMethod::Post
    }
    #[must_use]
    fn encoding() -> Encoding {
        Encoding::Json
    }
}

pub trait Execute {
    type Error;
    #[allow(clippy::missing_errors_doc)]
    fn execute<M: SlackMethod>(&self, method: M) -> std::result::Result<M::Response, Self::Error>;
}

pub struct Client {
    http: reqwest::blocking::Client,
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
            http: reqwest::blocking::Client::new(),
            base_url: base_url.into(),
            token: token.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn execute_internal<M: SlackMethod>(&self, method: M) -> Result<M::Response> {
        let body = method.into_body();
        let url = self.url(M::PATH);
        let req = match M::method() {
            HttpMethod::Post => self.http.post(url),
            HttpMethod::Get => self.http.get(url),
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
        self.execute_internal(method)
    }
}
