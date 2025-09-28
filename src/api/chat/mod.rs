pub mod post_ephemeral;
pub mod post_message;

use crate::client::Execute;

/// Entry point for Slack chat.* methods.
pub struct Chat<'a, C: Execute> {
    pub(crate) client: &'a C,
}

/// Extension trait to access grouped Slack Web API namespaces.
pub trait ChatExt: Execute + Sized {
    /// Access to `chat.*` methods.
    #[must_use]
    fn chat(&self) -> Chat<'_, Self> {
        Chat { client: self }
    }
}

impl<T: Execute> ChatExt for T {}
