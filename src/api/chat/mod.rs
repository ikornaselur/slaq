pub mod post_message;

use crate::client::Execute;

pub struct Chat<'a, C: Execute> {
    pub(crate) client: &'a C,
}

pub trait ChatExt: Execute + Sized {
    fn chat(&self) -> Chat<'_, Self> {
        Chat { client: self }
    }
}

impl<T: Execute> ChatExt for T {}
