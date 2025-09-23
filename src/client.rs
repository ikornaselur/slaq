use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json as json;
use std::time::Duration;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("decode error: {0}")]
    Decode(#[from] json::Error),
    #[error("rate limited, retry after {retry_after:?} {request_id:?}")]
    RateLimited {
        retry_after: Duration,
        request_id: Option<String>,
    },
    #[error("http status {code}: {body} {request_id:?}")]
    Status {
        code: reqwest::StatusCode,
        body: String,
        request_id: Option<String>,
    },
    #[error(transparent)]
    Slack(#[from] SlackError),
}

#[derive(Debug, Error)]
#[error("{code}")]
pub struct SlackError {
    pub code: String,
    pub warnings: Option<Vec<String>>,
    pub response_metadata: Option<json::Value>,
    pub request_id: Option<String>,
}

pub type Result<T> = std::result::Result<T, Error>;

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

        let request_id = resp
            .headers()
            .get("x-slack-req-id")
            .and_then(|v| v.to_str().ok())
            .map(std::string::ToString::to_string);

        let status = resp.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .map_or_else(|| Duration::from_secs(1), Duration::from_secs);
            return Err(Error::RateLimited {
                retry_after,
                request_id,
            });
        }

        let text = resp.text()?;
        if !status.is_success() {
            return Err(Error::Status {
                code: status,
                body: text,
                request_id,
            });
        }

        match json::from_str::<SlackApiResponse<M::Response>>(&text)? {
            SlackApiResponse::Ok { data, .. } => Ok(data),
            SlackApiResponse::Err {
                error,
                warnings,
                response_metadata,
                ..
            } => Err(Error::Slack(SlackError {
                code: error,
                warnings,
                response_metadata,
                request_id,
            })),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum SlackApiResponse<T> {
    Ok {
        ok: bool,
        #[serde(flatten)]
        data: T,
        #[serde(default)]
        warnings: Option<Vec<String>>,
        #[serde(default)]
        response_metadata: Option<json::Value>,
    },
    Err {
        ok: bool,
        error: String,
        #[serde(default)]
        warnings: Option<Vec<String>>,
        #[serde(default)]
        response_metadata: Option<json::Value>,
    },
}

impl Execute for Client {
    type Error = Error;
    fn execute<M: SlackMethod>(&self, method: M) -> Result<M::Response> {
        self.execute_internal(method)
    }
}
