/// Creates a `plain_text` object.
#[doc(hidden)]
#[macro_export]
macro_rules! plain {
    ($text:expr) => {
        $crate::blocks::PlainText::new($text)
    };
}

/// Creates an mrkdwn text object. Optional `verbatim = bool`.
#[doc(hidden)]
#[macro_export]
macro_rules! mrkdwn {
    ($text:expr $(, verbatim = $verbatim:expr)? $(,)?) => {{
        let mut value = $crate::blocks::MrkdwnText::new($text);
        $( value = value.verbatim($verbatim); )?
        value
    }};
}

/// Collects inline text objects into a `Vec<TextObject>` so callers don’t need `.into()`.
#[doc(hidden)]
#[macro_export]
macro_rules! fields {
    ( $($item:expr),+ $(,)? ) => {{
        let mut v: ::std::vec::Vec<$crate::blocks::TextObject> = ::std::vec::Vec::new();
        $( v.push(::core::convert::Into::<$crate::blocks::TextObject>::into($item)); )+
        v
    }};
    () => { ::std::vec::Vec::<$crate::blocks::TextObject>::new() };
}
