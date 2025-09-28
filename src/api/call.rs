use crate::client::{Execute, SlackMethod, SlackRequest};

#[must_use]
/// A fluent, type-safe builder for Slack API method calls.
///
/// Produced by extensions like `client.chat().post_message(...)`.
pub struct MethodCall<'a, C: Execute, M: SlackMethod> {
    pub(crate) client: &'a C,
    pub(crate) inner: M,
}

impl<C: Execute, M: SlackMethod> MethodCall<'_, C, M> {
    /// Builds a transport-agnostic request without sending it.
    ///
    /// Returns a `SlackRequest<M>` which exposes the path, method, encoding,
    /// and typed body. You can serialize it and send it using any HTTP client.
    pub fn build(self) -> SlackRequest<M> {
        SlackRequest {
            path: M::PATH,
            method: M::method(),
            encoding: M::encoding(),
            body: self.inner.into_body(),
        }
    }
    /// Sends the request using the attached client.
    ///
    /// # Errors
    /// Returns the client's error type if sending or decoding fails, or if Slack reports an error.
    pub fn send(self) -> Result<M::Response, C::Error> {
        self.client.execute(self.inner)
    }
}
