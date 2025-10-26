//! Public macros for ergonomic block construction.

#[macro_export]
macro_rules! plain {
    ($text:expr) => {
        $crate::blocks::PlainText::new($text)
    };
}

#[macro_export]
macro_rules! mrkdwn {
    ($text:expr $(, verbatim = $verbatim:expr)? $(,)?) => {{
        let mut value = $crate::blocks::MrkdwnText::new($text);
        $( value = value.verbatim($verbatim); )?
        value
    }};
}

#[macro_export]
macro_rules! header {
    ($text:expr $(, block_id = $block_id:expr)? $(,)?) => {{
        let mut builder = $crate::blocks::Header::new($crate::blocks::PlainText::new($text));
        $( builder = builder.block_id($block_id); )?
        builder.build()
    }};
}

#[macro_export]
macro_rules! divider {
    ($(block_id = $block_id:expr)? $(,)?) => {{
        let mut builder = $crate::blocks::Divider::new();
        $( builder = builder.block_id($block_id); )?
        builder.build()
    }};
}

#[macro_export]
macro_rules! section {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut builder = $crate::blocks::Section::new();
        $( section!(@apply builder, $key, $value); )+
        builder.build()
    }};
    (@apply $builder:ident, text, $value:expr) => {
        $builder = $builder.text($value);
    };
    (@apply $builder:ident, fields, $value:expr) => {
        $builder = $builder.fields($value);
    };
    (@apply $builder:ident, accessory, $value:expr) => {
        $builder = $builder.accessory($value);
    };
    (@apply $builder:ident, expand, $value:expr) => {
        $builder = $builder.expand($value);
    };
    (@apply $builder:ident, block_id, $value:expr) => {
        $builder = $builder.block_id($value);
    };
    (@apply $builder:ident, $unexpected:ident, $_value:expr) => {
        compile_error!(concat!("unsupported section! argument: ", stringify!($unexpected)));
    };
}

/// Collects inline text objects into a `Vec<TextObject>` so callers don’t need `.into()`.
#[macro_export]
macro_rules! fields {
    ( $($item:expr),+ $(,)? ) => {{
        let mut v: ::std::vec::Vec<$crate::blocks::TextObject> = ::std::vec::Vec::new();
        $( v.push(::core::convert::Into::<$crate::blocks::TextObject>::into($item)); )+
        v
    }};
    () => { ::std::vec::Vec::<$crate::blocks::TextObject>::new() };
}

#[macro_export]
macro_rules! video {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut __title: Option<$crate::blocks::PlainText> = None;
        let mut __video_url: Option<String> = None;
        let mut __thumb: Option<String> = None;
        let mut __alt: Option<String> = None;
        let mut __description: Option<$crate::blocks::PlainText> = None;
        let mut __title_url: Option<String> = None;
        let mut __provider_icon_url: Option<String> = None;
        let mut __provider_name: Option<String> = None;
        let mut __author_name: Option<String> = None;
        $(
            video!(@assign __title, __video_url, __thumb, __alt, __description, __title_url, __provider_icon_url, __provider_name, __author_name; $key = $value);
        )+
        let title = __title.expect("video! macro requires `title = ...`");
        let video_url = __video_url.expect("video! macro requires `video_url = ...`");
        let thumbnail_url = __thumb.expect("video! macro requires `thumbnail_url = ...`");
        let alt_text = __alt.expect("video! macro requires `alt_text = ...`");
        let mut builder = $crate::blocks::Video::new(title, video_url, thumbnail_url, alt_text);
        if let Some(desc) = __description { builder = builder.description(desc); }
        if let Some(url) = __title_url { builder = builder.title_url(url); }
        if let Some(icon) = __provider_icon_url { builder = builder.provider_icon_url(icon); }
        if let Some(name) = __provider_name { builder = builder.provider_name(name); }
        if let Some(author) = __author_name { builder = builder.author_name(author); }
        builder.build()
    }};
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; title = $value:expr) => {
        $title = Some($crate::blocks::PlainText::new($value));
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; video_url = $value:expr) => {
        $video_url = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; thumbnail_url = $value:expr) => {
        $thumb = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; alt_text = $value:expr) => {
        $alt = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; description = $value:expr) => {
        $desc = Some($crate::blocks::PlainText::new($value));
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; title_url = $value:expr) => {
        $title_url = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; provider_icon_url = $value:expr) => {
        $icon = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; provider_name = $value:expr) => {
        $provider_name = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; author_name = $value:expr) => {
        $author = Some($value.into());
    };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident; $unexpected:ident = $_value:expr) => {
        compile_error!(concat!("unsupported video! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! blocks {
    ($($block:expr),+ $(,)?) => {{
        let mut items = ::std::vec::Vec::new();
        $( items.push($block?); )+
        ::std::result::Result::Ok(items)
    }};
    () => { ::std::result::Result::Ok(::std::vec::Vec::new()) };
}
