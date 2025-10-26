use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json as json;
use thiserror::Error;

#[cfg(feature = "transport-reqwest")]
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
/// HTTP methods supported by the Slack Web API wrapper.
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Copy, Clone, Debug)]
/// Wire encoding used when sending bodies.
pub enum Encoding {
    Json,
}

/// A Slack API method definition implemented by generated request types.
pub trait SlackMethod {
    /// The Slack API path (e.g., "/chat.postMessage").
    const PATH: &'static str;
    /// The serializable request body type.
    type Body: Serialize;
    /// The deserializable response body type.
    type Response: DeserializeOwned;
    /// Converts the method into its serializable body.
    fn into_body(self) -> Self::Body;
    /// The HTTP method to use when sending this request.
    #[must_use]
    fn method() -> HttpMethod {
        HttpMethod::Post
    }
    /// The wire encoding to use when sending this request.
    #[must_use]
    fn encoding() -> Encoding {
        Encoding::Json
    }
}

/// A transport that can execute Slack API methods.
pub trait Execute {
    type Error;
    /// Executes the provided Slack API method and returns the decoded response.
    ///
    /// # Errors
    /// Returns a transport-specific error if the request fails to send, decode, or if Slack reports an error.
    fn execute<M: SlackMethod>(&self, method: M) -> std::result::Result<M::Response, Self::Error>;
}

/// A built, transport-agnostic Slack API request.
#[must_use]
pub struct SlackRequest<M: SlackMethod> {
    pub path: &'static str,
    pub method: HttpMethod,
    pub encoding: Encoding,
    pub body: M::Body,
}

impl<M: SlackMethod> SlackRequest<M> {
    /// Returns the HTTP Content-Type for this request's encoding.
    #[must_use]
    pub fn content_type(&self) -> &'static str {
        match self.encoding {
            Encoding::Json => "application/json",
        }
    }
    /// Serializes the request body to a JSON string.
    ///
    /// # Errors
    /// Returns a serialization error if the body cannot be encoded as JSON.
    pub fn to_json(&self) -> json::Result<String> {
        match self.encoding {
            Encoding::Json => json::to_string(&self.body),
        }
    }
}

impl<M: SlackMethod> From<M> for SlackRequest<M> {
    fn from(method: M) -> Self {
        SlackRequest {
            path: M::PATH,
            method: M::method(),
            encoding: M::encoding(),
            body: method.into_body(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slack_request_json_and_content_type() {
        #[derive(serde::Serialize)]
        struct Body {
            a: u8,
        }

        struct M;
        impl SlackMethod for M {
            const PATH: &'static str = "/x";
            type Body = Body;
            type Response = ();
            fn into_body(self) -> Self::Body {
                Body { a: 1 }
            }
        }

        let req: SlackRequest<M> = M.into();
        assert_eq!(req.content_type(), "application/json");
        let json = req.to_json().expect("json");
        assert_eq!(json, "{\"a\":1}");
    }
}

#[cfg(feature = "transport-reqwest")]
/// Blocking Slack Web API client using reqwest as the transport.
pub struct Client {
    http: reqwest::blocking::Client,
    base_url: String,
    token: String,
}

#[cfg(feature = "transport-reqwest")]
/// Errors returned by the reqwest-based client.
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
/// Error returned by Slack when `ok: false`.
pub struct SlackError {
    pub code: String,
    pub warnings: Option<Vec<String>>,
    pub response_metadata: Option<json::Value>,
    pub request_id: Option<String>,
}

#[cfg(feature = "transport-reqwest")]
/// Convenience result alias for the reqwest-based client.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "transport-reqwest")]
impl Client {
    /// Creates a new blocking client using the given base Slack API URL and bearer token.
    #[must_use]
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
        let req = SlackRequest::<M> {
            path: M::PATH,
            method: M::method(),
            encoding: M::encoding(),
            body: method.into_body(),
        };
        self.send(&req)
    }

    /// Sends a previously built `SlackRequest` using this client.
    ///
    /// # Errors
    /// - `Error::Http` if the underlying HTTP request fails.
    /// - `Error::RateLimited` if Slack responds with 429 and a Retry-After header.
    /// - `Error::Status` for non-success HTTP statuses.
    /// - `Error::Decode` if response JSON cannot be decoded.
    /// - `Error::Slack` if Slack returns `ok: false` with an error code.
    pub fn send<M: SlackMethod>(&self, request: &SlackRequest<M>) -> Result<M::Response> {
        let url = self.url(request.path);
        let req = match request.method {
            HttpMethod::Post => self.http.post(url),
            HttpMethod::Get => self.http.get(url),
        }
        .bearer_auth(&self.token);

        let resp = match request.encoding {
            Encoding::Json => req.json(&request.body).send()?,
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

#[cfg(feature = "transport-reqwest")]
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

#[cfg(feature = "transport-reqwest")]
impl Execute for Client {
    type Error = Error;
    fn execute<M: SlackMethod>(&self, method: M) -> Result<M::Response> {
        self.execute_internal(method)
    }
}
