use crate::client::{Execute, SlackMethod};

#[must_use]
pub struct MethodCall<'a, C: Execute, M: SlackMethod> {
    pub(crate) client: &'a C,
    pub(crate) inner: M,
}

impl<'a, C: Execute, M: SlackMethod> MethodCall<'a, C, M> {
    #[allow(clippy::missing_errors_doc)]
    pub fn send(self) -> Result<M::Response, C::Error> {
        self.client.execute(self.inner)
    }
}
