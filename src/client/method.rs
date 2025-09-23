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
    fn method() -> HttpMethod {
        HttpMethod::Post
    }
    fn encoding() -> Encoding {
        Encoding::Json
    }
}

pub trait Execute {
    type Error;
    fn execute<M: SlackMethod>(&self, method: M) -> Result<M::Response, Self::Error>;
}
